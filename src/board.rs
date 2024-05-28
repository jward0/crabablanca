use tokio::runtime::EnterGuard;

use crate::bit_functions::{bidirectional_shift, bishop_move_mask, block_ray, get_lsb, iterate_over, king_move_mask, knight_move_mask, move_piece, pawn_capture_mask, queen_move_mask, rook_move_mask};
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
            // white_pawns:   0x000000000000FF00,
            white_pawns:   0x0000000000000000,
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

            // all_white:     0x000000000000FFFF,
            all_white:     0x00000000000000FF,
            all_black:     0xFFFF000000000000,
            // all_pieces:    0xFFFF00000000FFFF,
            all_pieces:    0xFFFF0000000000FF,

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
        let knights: Vec<u64>;
        let bishops: Vec<u64>;
        let rooks: Vec<u64>;
        let queens: Vec<u64>;
        let king: u64;
        let own_pieces: u64;
        let enemy_pieces: u64;

        if self.to_move == 1 {
            // White to move
            pawns = iterate_over(self.white_pawns);
            pawn_start_row = RANK_2;
            pawn_promote_row = RANK_8;
            knights = iterate_over(self.white_knights);
            bishops = iterate_over(self.white_bishops);
            rooks = iterate_over(self.white_rooks);
            queens = iterate_over(self.white_queens);
            king = self.white_king;
            own_pieces = self.all_white;
            enemy_pieces = self.all_black;
        } else {
            // Black to move
            pawns = iterate_over(self.black_pawns);
            pawn_start_row = RANK_7;
            pawn_promote_row = RANK_1;
            knights = iterate_over(self.black_knights);
            bishops = iterate_over(self.black_bishops);
            rooks = iterate_over(self.black_rooks);
            queens = iterate_over(self.black_queens);
            king = self.black_king;
            own_pieces = self.all_black;
            enemy_pieces = self.all_white;
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
                if (capture & enemy_pieces) != 0 {
                    pawn_moves.push(*capture)
                }
            }

            // En passant not yet implemented
            
            for pawn_move in pawn_moves.iter() {
                move_list.push(self.apply_move(*pawn, *pawn_move));
            }

        }

        // Generate knight moves

        for knight in knights.iter() {

            let move_mask: u64 = knight_move_mask(*knight, own_pieces);
            for move_ in iterate_over(move_mask).iter() {
                move_list.push(self.apply_move(*knight, *move_));
            }
        }

        // Generate bishop moves

        for bishop in bishops.iter() {

            let move_mask: u64 = bishop_move_mask(*bishop, own_pieces, enemy_pieces);

            for move_ in iterate_over(move_mask).iter() {
                move_list.push(self.apply_move(*bishop, *move_));
            }
        }

        // Generate rook moves

        for rook in rooks.iter() {

            let move_mask: u64 = rook_move_mask(*rook, own_pieces, enemy_pieces);

            for move_ in iterate_over(move_mask).iter() {
                move_list.push(self.apply_move(*rook, *move_));
            }
        }

        // Generate queen moves

        for queen in queens.iter() {

            let move_mask: u64 = queen_move_mask(*queen, own_pieces, enemy_pieces);

            for move_ in iterate_over(move_mask).iter() {
                move_list.push(self.apply_move(*queen, *move_));
            }
        }

        // Generate king moves 

        let move_mask: u64 = king_move_mask(king, own_pieces);

        for move_ in iterate_over(move_mask).iter() {
            move_list.push(self.apply_move(king, *move_));
        }

        // Generate castling moves

        move_list

    }
}
