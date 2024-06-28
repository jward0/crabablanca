use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::board::Board;
use crate::bit_functions::{count_bits};

fn evaluate(board: &Board) -> f64 {

    // Check checkmates

    let checks = board.check_check();
    let checkmates = board.check_checkmate(checks);

    if checkmates.0 == true {
        return f64::MIN
    } else if checkmates.1 == true {
        return f64::MAX
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

    return material_advantage
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
    
    pub fn search_n_plys(node_cell:&Rc<RefCell<Node>>, n: usize) {
        
        for i in 0..n {
            Node::search_next_ply(node_cell);
        }
        Node::pull_best_up(node_cell);
    }

    fn search_next_ply(node_cell: &Rc<RefCell<Node>>) {

        let node = &mut *node_cell.borrow_mut();

        if !node.children.is_empty() {
            for child in node.children.iter() {
                Node::search_next_ply(&child);
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