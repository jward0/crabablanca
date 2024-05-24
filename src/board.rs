use crate::bit_functions::{get_lsb, bidirectional_shift, pawn_capture_mask, iterate_over, move_piece};
use crate::constants::*;

#[derive(Clone, Debug)]
pub struct Board {

    pub white_pawns:   u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_rooks:   u64,
    pub white_queens:  u64,
    pub white_king:    u64,

    pub black_pawns:   u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_rooks:   u64,
    pub black_queens:  u64,
    pub black_king:    u64,

    pub all_white:     u64,
    pub all_black:     u64,
    pub all_pieces:    u64,

    pub to_move:       u8 // 1 for white to move, 0 for black to move

}

impl Board {
    pub fn new() -> Board {
        Board {
            white_pawns:   0x000000000000FF00,
            white_knights: 0x0000000000000042,
            white_bishops: 0x0000000000000024,
            white_rooks:   0x0000000000000081,
            white_queens:  0x0000000000000008,
            white_king:    0x0000000000000010,
        
            black_pawns:   0x00FF000000000000,
            black_knights: 0x4200000000000000,
            black_bishops: 0x2400000000000000,
            black_rooks:   0x8100000000000000,
            black_queens:  0x0800000000000000,
            black_king:    0x1000000000000000,

            all_white:     0x000000000000FFFF,
            all_black:     0xFFFF000000000000,
            all_pieces:    0xFFFF00000000FFFF,

            to_move:       1
        }
    }

    fn apply_move(&self, from: u64, to: u64) -> Board {
        Board {
            white_pawns:   move_piece(self.white_pawns, from, to),
            white_knights: move_piece(self.white_knights, from, to),
            white_bishops: move_piece(self.white_bishops, from, to),
            white_rooks:   move_piece(self.white_rooks, from, to),
            white_queens:  move_piece(self.white_queens, from, to),
            white_king:    move_piece(self.white_king, from, to),
            black_pawns:   move_piece(self.black_pawns, from, to),
            black_knights: move_piece(self.black_knights, from, to),
            black_bishops: move_piece(self.black_bishops, from, to),
            black_rooks:   move_piece(self.black_rooks, from, to),
            black_queens:  move_piece(self.black_queens, from, to),
            black_king:    move_piece(self.black_king, from, to),
            all_white:     move_piece(self.all_white, from, to),
            all_black:     move_piece(self.all_black, from, to),
            all_pieces:    move_piece(self.all_pieces, from, to),
            to_move: self.to_move ^ (1 << 7) 
        }
    }

    fn is_legal_position(&self) -> bool {
        true
    }

    pub fn generate_move_list(&self) -> Vec<Board> {

        let pawns: Vec<u64>;
        let pawn_start_row: u64;
        let pawn_promote_row: u64;
        let valid_captures: u64;

        if self.to_move == 1 {
            // White to move
            pawns = iterate_over(self.white_pawns);
            pawn_start_row = RANK_2;
            pawn_promote_row = RANK_8;
            valid_captures = self.all_black;
        } else {
            // Black to move
            pawns = iterate_over(self.black_pawns);
            pawn_start_row = RANK_7;
            pawn_promote_row = RANK_1;
            valid_captures = self.all_white;
        }

        let mut move_list: Vec<Board> = vec![];

        // Generate pawn moves

        // Moves

        for pawn in pawns.iter() {

            let mut pawn_moves: Vec<u64> = vec![];

            // Single moves

            let single_move: u64 = bidirectional_shift(*pawn, 8, self.to_move);
        
            if (self.all_pieces & single_move) == 0 {
                // Check for promotion
                if (single_move & pawn_promote_row) != 0 {
                    // TODO: IMPLEMENT PROMOTION
                } else {
                    pawn_moves.push(single_move)
                }
            }
            // Double moves
            if (pawn & pawn_start_row) != 0 {
                let double_move: u64 = bidirectional_shift(*pawn, 16, self.to_move);
                if (self.all_pieces & double_move) == 0 {
                    pawn_moves.push(double_move)
                }
            }

            // Standard captures

            let capture_mask: u64 = pawn_capture_mask(*pawn, self.to_move);
            for capture in iterate_over(capture_mask).iter() {
                if (capture & valid_captures) == 1 {
                    pawn_moves.push(*capture)
                }
            }

            // En passant not yet implemented
            
            for pawn_move in pawn_moves.iter() {
                move_list.push(self.apply_move(*pawn, *pawn_move));
            }

        }

        // Generate knight moves

        // Generate bishop moves

        // Generate rook moves

        // Generate queen moves

        // Generate king moves 

        // Generate castling moves

        move_list

    }
}
