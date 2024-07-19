use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::board::Board;
use crate::bit_functions::count_bits;
use crate::constants::*;

fn evaluate(board: &Board) -> f64 {

    // Check checkmates

    let checks = board.check_check();
    let checkmates = board.check_checkmate(checks);

    if checkmates.0 == true {
        return -9999.0
    } else if checkmates.1 == true {
        return 9999.0
    }
    
    // Count material balance
    // Slightly unusual structure/casting here is to establish material advantages as float
    // In case of future changes to method of piece valuation
    let white_material: f64 = (1 * count_bits(board.white_pawns) + 
                               3 * count_bits(board.white_bishops) +
                               3 * count_bits(board.white_knights) + 
                               5 * count_bits(board.white_rooks) + 
                               9 * count_bits(board.white_queens)) as f64;

    let black_material: f64 = (1 * count_bits(board.black_pawns) + 
                               3 * count_bits(board.black_bishops) +
                               3 * count_bits(board.black_knights) + 
                               5 * count_bits(board.black_rooks) + 
                               9 * count_bits(board.black_queens)) as f64;

    let material_advantage: f64 = white_material - black_material;

    // Assess centrality
    let white_central_pawns: f64 = count_bits(board.white_pawns & CENTRE) as f64;
    let black_central_pawns: f64 = count_bits(board.black_pawns & CENTRE) as f64;

    let centrality_advantage: f64 = 0.1 * (white_central_pawns - black_central_pawns);

    // Encourage attacking play
    let checks_advantage: f64 = checks.1 as u8 as f64 - checks.0 as u8 as f64;

    // Encourage development
    let wb: Board = Board {
        to_move : 1,
        ..*board
    };
    let bb: Board = Board {
        to_move : 0,
        ..*board
    };
    let white_mobility = wb.generate_move_list().len();
    let black_mobility = bb.generate_move_list().len();
    let mobility_advantage: f64 = 0.1 * (white_mobility as f64 - black_mobility as f64);

    // Encourage king safety

    return material_advantage + centrality_advantage + checks_advantage + mobility_advantage;
} 


pub struct Node {
    pub depth: usize,
    pub board: Board,
    pub static_eval: f64,
    pub deep_eval: f64,
    pub best_next_move: Option<Board>,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>
}

impl Node {

    pub fn new(board: &Board) -> Node {
        let eval = evaluate(board);
        Node {
            depth: 0,
            board: board.clone(),
            static_eval: eval,
            deep_eval: eval,
            best_next_move: None,
            parent: None,
            children: vec![]
        }
    }

    fn add_child(parent: &mut Node, child: Rc<RefCell<Node>>) {
        parent.children.push(child);
    }
    
    fn propagate_eval_up(node_cell: &Rc<RefCell<Node>>, deep_eval: f64) {
        let node: &mut Node = &mut *node_cell.borrow_mut();
        node.deep_eval = deep_eval;
        if let Some(weak_parent) = &node.parent {
            if let Some(parent) = weak_parent.upgrade() {
                Node::propagate_eval_up(&parent, deep_eval);
            }
        }
    }

    fn propagate_best_next_move_up(node_cell: &Rc<RefCell<Node>>, next_move: &Board) {
        let node: &mut Node = &mut *node_cell.borrow_mut();
        node.best_next_move = Some(next_move.clone());
        if let Some(weak_parent) = &node.parent {
            if let Some(parent) = weak_parent.upgrade() {
                Node::propagate_best_next_move_up(&parent, next_move);
            }
        }
    }

    fn pull_best_up(node_cell: &Rc<RefCell<Node>>) {

        let node = &mut *node_cell.borrow_mut();

        if node.children.is_empty() {
            return
        }

        let mut best_eval: f64 = if node.board.to_move == 1 {f64::MIN } else {f64::MAX};
        let mut best_board = Board::new();

        for child in node.children.iter() {

            Node::pull_best_up(child);

            let cb = child.borrow();
            if node.board.to_move == 1 {
                if cb.deep_eval > best_eval {
                    best_eval = cb.deep_eval;
                    best_board = cb.board.clone();
                }
            } else {
                if node.deep_eval < best_eval {
                    best_eval = cb.deep_eval;
                    best_board = cb.board.clone();
                }
            }
        }

        node.deep_eval = best_eval;
        node.best_next_move = Some(best_board);

    }

    pub fn process_node_cell(node_cell: &Rc<RefCell<Node>>, depth: usize) {

        let (deep_eval, next_move) = Node::get_ab_eval(node_cell, depth, f64::MAX, f64::MIN);
        let node = &mut *node_cell.borrow_mut();
        node.deep_eval = deep_eval;
        node.best_next_move = Some(next_move);
    }

    pub fn get_ab_eval(node_cell: &Rc<RefCell<Node>>, depth: usize, alpha: f64, beta: f64) -> (f64, Board) {

        let mut local_alpha: f64 = alpha;
        let mut local_beta: f64 = beta;

        let node = &mut *node_cell.borrow_mut();

        if node.depth == depth {
            return (node.static_eval, node.board);
        } else {

            let mut best_eval: f64 = if node.board.to_move == 1 {f64::MIN} else {f64::MAX};
            let mut best_board: Board = Board::new();

            let move_list: Vec<Board> = node.board.generate_move_list();

            for board in move_list.iter().rev() {
                let static_eval: f64 = evaluate(&board);

                let tmpcell = Rc::new(RefCell::new(
                    Node {
                        depth: node.depth + 1,
                        board: board.clone(),
                        static_eval: static_eval,
                        deep_eval: 0.0,
                        best_next_move: None,
                        parent: Some(Rc::downgrade(&node_cell)),
                        children: vec![]
                    }
                ));

                if node.board.to_move == 1 {
                    let (move_eval, _) = Node::get_ab_eval(&tmpcell, depth, f64::MAX, best_eval);
                    Node::add_child(node, tmpcell);
                    if move_eval >= best_eval {
                        best_eval = move_eval;
                        best_board = *board;
                    }
                    if move_eval > local_alpha {
                        break
                    }

                } else {
                    let (move_eval, _) = Node::get_ab_eval(&tmpcell, depth, best_eval, f64::MIN);
                    Node::add_child(node, tmpcell);
                    if move_eval <= best_eval {
                        best_eval = move_eval;
                        best_board = *board;
                    }
                    if move_eval < local_beta {
                        break
                    }
                }
            }
            return (best_eval, best_board)
        }
    }

    // pub fn n_ply_dfs(node_cell: &Rc<RefCell<Node>>, n: usize) -> f64 {

    //     let node = &mut *node_cell.borrow_mut();

    //     if node.depth == n {
    //         return node.static_eval;
    //     } else {

    //         let alpha: f64 = node.static_eval;
    //         let beta: f64 = node.static_eval;

    //         let move_list: Vec<Board> = node.board.generate_move_list();

    //         for board in move_list.iter() {
    //             let static_eval: f64 = evaluate(&board);

    //             let mut child_node = Node {
    //                     depth: node.depth + 1,
    //                     board: board.clone(),
    //                     static_eval: static_eval,
    //                     deep_eval: static_eval,
    //                     best_next_move: None,
    //                     parent: Some(Rc::downgrade(&node_cell)),
    //                     children: vec![]
    //                 };

    //             child_node

    //             Node::add_child(
    //                 node,
    //                 child_node
    //             );
    //             return Node::n_ply_dfs(&node.children.last().unwrap(), n);
    //         }
    //         return 0.0;
    //     }
    // }
    
    pub fn search_n_plys(node_cell:&Rc<RefCell<Node>>, n: usize) {

        let alpha: f64 = node_cell.borrow().static_eval;
        let beta: f64 = node_cell.borrow().static_eval;
        
        for _ in 0..n {
            Node::search_next_ply(node_cell, alpha, beta);
        }
        Node::pull_best_up(node_cell);
    }

    fn search_next_ply(node_cell: &Rc<RefCell<Node>>, alpha: f64, beta: f64) {

        let node = &mut *node_cell.borrow_mut();

        if !node.children.is_empty() {
            for child in node.children.iter() {
                Node::search_next_ply(&child, alpha, beta);
            }
        }

        let move_list: Vec<Board> = node.board.generate_move_list();
        
        for board in move_list.iter() {

            let static_eval = evaluate(&board);
            Node::add_child(
                node,
                Rc::new(RefCell::new(
                    Node {
                        depth: node.depth + 1,
                        board: board.clone(),
                        static_eval: static_eval,
                        deep_eval: static_eval,
                        best_next_move: None,
                        parent: Some(Rc::downgrade(&node_cell)),
                        children: vec![]
                    }
                ))
            );
        }
    }
}