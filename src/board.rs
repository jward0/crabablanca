use std::num::NonZeroI128;

use tokio::runtime::EnterGuard;

use crate::bit_functions::{bidirectional_shift, bishop_move_mask, get_rank_or_file, coord_to_bit, count_bits, get_lsb, get_msb, iterate_over, king_move_mask, knight_move_mask, move_piece, pawn_capture_mask, queen_move_mask, rook_move_mask};
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
            to_move: self.to_move ^ 1
        }
    }

    fn is_legal_position(&self) -> bool {
        true
    }

    fn get_pieces(&self, piece_type: char, to_move: u8) -> u64 {
        match piece_type {
            'p' => if to_move == 1 {self.white_pawns} else {self.black_pawns},
            'n' => if to_move == 1 {self.white_knights} else {self.black_knights},
            'b' => if to_move == 1 {self.white_bishops} else {self.black_bishops},
            'r' => if to_move == 1 {self.white_rooks} else {self.black_rooks},
            'q' => if to_move == 1 {self.white_queens} else {self.black_queens},
            'k' => if to_move == 1 {self.white_king} else {self.black_king},
            _ => unreachable!()
        }
    }

    fn reverse_move_mask(self, piece_type: char, to_move: u8, to: u64) -> u64 {

        // Never used for pawns, do not use for pawns
        assert_ne!(piece_type, 'p');

        let possible_pieces: u64 = self.get_pieces(piece_type, to_move);
        let own_pieces: u64;
        let enemy_pieces: u64;

        if to_move == 1 {
            own_pieces = self.all_white;
            enemy_pieces = self.all_black;
        } else {
            own_pieces = self.all_black;
            enemy_pieces = self.all_black;
        }

        // own and enemy pieces are flipped since we want to know where they could've come from
        // rather than where they could go
        match piece_type {
            'n' => knight_move_mask(to, enemy_pieces),
            'b' => bishop_move_mask(to, enemy_pieces, own_pieces),
            'r' => rook_move_mask(to, enemy_pieces, own_pieces),
            'q' => queen_move_mask(to, enemy_pieces, own_pieces),
            'k' => king_move_mask(to, enemy_pieces),
            _ => unreachable!()
        }
    }

    fn parse_input(&self, input: &String) -> Option<Board> {

        if !input.is_ascii() {
            return None
        }

        let inlen = input.len();

        let target: String = input.drain(inlen-2..).collect();
        let rank: u16 = target.chars().collect::<Vec<char>>()[1].to_digit(10)? as u16;
        let file: u16 = target.chars().collect::<Vec<char>>()[0].to_ascii_lowercase() as u16 - 61;

        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        let piece_type: char;
        let disambiguation: u64;

        let is_capture: bool = input.chars().collect::<Vec<char>>().contains(&'x');

        // Get piece type
        if inlen > 2 {
            piece_type = input.chars().collect::<Vec<char>>()[0].to_ascii_lowercase();
            if !['b', 'n', 'r', 'q', 'k'].contains(&piece_type) {
                return None
            }
        } else {
            piece_type = 'p';
        }

        // Get possible disambiguation information

        let disambiguation: u64;

        if (inlen == 4 && !is_capture) || (inlen == 5 && is_capture) {

            let disambig_char = input.chars().collect::<Vec<char>>()[1];

            disambiguation = get_rank_or_file(disambig_char);

            if disambiguation == 0 {return None}

        } else if (inlen == 5 && !is_capture) || (inlen == 6 && is_capture) {

            let disambig_char_1 = input.chars().collect::<Vec<char>>()[1];
            let disambig_char_2 = input.chars().collect::<Vec<char>>()[2];

            let dbg1 = get_rank_or_file(disambig_char_1);
            let dbg2 = get_rank_or_file(disambig_char_2);

            if dbg1 == 0 || dbg2 == 0 {return None}

            disambiguation = dbg1 | dbg2;
        }

        let available_pieces: u64 = self.get_pieces(piece_type, self.to_move);

        let to: u64 = coord_to_bit((rank, file));
        let from: u64;

        match inlen {
            
            2 => {
                // Unambigious pawn move
                let available_pawns: u64;
                if self.to_move == 1 {
                    available_pawns = (FILE_A << file) &  self.white_pawns;
                    from = get_msb(available_pawns);
                } else {
                    available_pawns = (FILE_A << file) &  self.black_pawns;
                    from = get_lsb(available_pawns);
                }
            },
            3 => {
                // Unambigious piece move
                from = self.reverse_move_mask(piece_type, self.to_move, to) & available_pieces;
            },
            4 => {
                // Unambiguous capture or ambiguous move
                if is_capture {
                    from = self.reverse_move_mask(piece_type, self.to_move, to) & available_pieces;
                } else {

                }
            },
            5 => {
                // Ambiguous capture or double ambiguous move
                if is_capture {

                } else {

                }
            },
            6 => {
                // Double ambiguous capture
            },
            _ => return None
        }

        if from == 0 {
            return None;
        } else if count_bits(from) > 1 {
            return None;
        } 

        Some(self.apply_move(from, to))
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
