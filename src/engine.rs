use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crate::board::Board;
use crate::bit_functions::{count_bits};

fn evaluate(board: &Board) -> f64 {

    // Check checkmates

    let checks = board.check_check();
    let checkmates = board.check_checkmate(checks);

    if checkmates.0 {
        return f64::MIN
    } else if checkmates.1 {
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

struct TranspositionTable {

}

struct Node {
    pub depth: usize,
    pub board: Board,
    pub static_eval: f64,
    pub deep_eval: f64,
    pub parent: Option<Weak<RefCell<Node>>>,
    pub children: Vec<Rc<RefCell<Node>>>
}

impl Node {

    fn add_child(parent: &Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        parent.borrow_mut().children.push(child);
    }

    fn propagate_eval_up(node_cell: Rc<RefCell<Node>>, deep_eval: f64) {
        let node = &mut *node_cell.borrow_mut();
        node.deep_eval = deep_eval;
        if let Some(weak_parent) = &node.parent {
            if let Some(parent) = weak_parent.upgrade() {
                Node::propagate_eval_up(parent, deep_eval);
            }
        }
    }

    fn search_next_ply(node_cell: Rc<RefCell<Node>>) {
        let node = &mut *node_cell.borrow_mut();

        let move_list: Vec<Board> = node.board.generate_move_list();
        let mut best_static_eval: f64 = if node.board.to_move == 0 {f64::MIN } else {f64::MAX};

        for board in move_list.iter() {
            let static_eval = evaluate(&board);
            if node.board.to_move == 0 {
                if static_eval > best_static_eval {best_static_eval = static_eval};
            } else {
                if static_eval < best_static_eval {best_static_eval = static_eval};
            }

            Node::add_child(
                &node_cell,
                Rc::new(RefCell::new(
                    Node {
                        depth: node.depth + 1,
                        board: board.clone(),
                        static_eval: static_eval,
                        deep_eval: static_eval,
                        parent: Some(Rc::downgrade(&node_cell)),
                        children: vec![]
                    }
                )));
        }
    }
    
}