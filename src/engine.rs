use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::board::Board;
use crate::bit_functions::{count_bits, king_forward_mask};
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
    let checks_advantage: f64 = 0.5* (checks.1 as u8 as f64 - checks.0 as u8 as f64);

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
    let white_castle_potential = board.white_castle_flags.0 as u32 + board.white_castle_flags.1 as u32;
    let white_king_shield = count_bits(king_forward_mask(board.white_king, 1) & board.white_pawns) as u32;

    let black_castle_potential = board.black_castle_flags.0 as u32 + board.black_castle_flags.1 as u32;
    let black_king_shield = count_bits(king_forward_mask(board.black_king, 0) & board.black_pawns) as u32;
    // let white_king_tropism: u32 = iterate_over(board.all_black).into_iter().map(|b| {manhattan_distance(board.white_king, b)}).sum();
    // let black_king_tropism: u32 = iterate_over(board.all_white).into_iter().map(|b| {manhattan_distance(board.black_king, b)}).sum();

    let king_safety_advantage = (white_castle_potential + white_king_shield - black_castle_potential - black_king_shield) as f64;

    // Penalise doubled pawns
    let white_doubled_pawns: u8 = (0..8).collect::<Vec<usize>>().iter().map(|i| (((FILE_A << i) & board.white_pawns) > 1) as u8).sum();
    let black_doubled_pawns: u8 = (0..8).collect::<Vec<usize>>().iter().map(|i| (((FILE_A << i) & board.black_pawns) > 1) as u8).sum();

    let doubled_pawn_advantage: f64 = 0.5 * (white_doubled_pawns - black_doubled_pawns) as f64;

    return material_advantage + centrality_advantage + checks_advantage + mobility_advantage + king_safety_advantage + doubled_pawn_advantage;
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

    pub fn process_node_cell(node_cell: &Rc<RefCell<Node>>, depth: usize) {

        // Carry out deep a/b eval and find best next move for node

        let (deep_eval, next_move) = Node::get_ab_eval(node_cell, depth, f64::MAX, f64::MIN);
        let node = &mut *node_cell.borrow_mut();
        node.deep_eval = deep_eval;
        node.best_next_move = Some(next_move);
    }

    pub fn get_ab_eval(node_cell: &Rc<RefCell<Node>>, depth: usize, alpha: f64, beta: f64) -> (f64, Board) {

        let local_alpha: f64 = alpha;
        let local_beta: f64 = beta;

        let node = &mut *node_cell.borrow_mut();

        if node.depth == depth {
            // Eval at stopping depth is just static eval
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

                // Alpha-beta eval
                // White won't choose a move that black has a powerful response to and vice-versa
                // Neither side will play hope chess, will select moves assuming the other side
                // will choose best refutation

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
}