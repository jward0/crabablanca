use crate::bit_functions::{bidirectional_shift, bishop_move_mask, coord_to_bit, count_bits, get_bit_rf, get_rank_or_file, iterate_over, king_move_mask, knight_move_mask, move_piece, pawn_capture_mask, queen_move_mask, rook_move_mask};
use crate::constants::*;

#[derive(Clone, Debug)]
pub struct Board {

    pub white_pawns:     u64,
    pub white_knights:   u64,
    pub white_bishops:   u64,
    pub white_rooks:     u64,
    pub white_queens:    u64,
    pub white_king:      u64,

    pub black_pawns:     u64,
    pub black_knights:   u64,
    pub black_bishops:   u64,
    pub black_rooks:     u64,
    pub black_queens:    u64,
    pub black_king:      u64,

    pub all_white:       u64,
    pub all_black:       u64,
    pub all_pieces:      u64,

    pub white_check:     bool,
    pub black_check:     bool,

    pub white_checkmate: bool,
    pub black_checkmate: bool,

    pub to_move:         u8 // 1 for white to move, 0 for black to move

}

impl Board {
    pub fn new() -> Board {
        Board {
            // white_pawns:     0x000000000000FF00,
            white_pawns:     0x0000000000000000,
            white_knights:   0x0000000000000042,
            white_bishops:   0x0000000000000024,
            white_rooks:     0x0000000000000081,
            white_queens:    0x0000000000000008,
            white_king:      0x0000000000000010,
        
            // black_pawns:     0x00FF000000000000,
            black_pawns:     0x0000000000000000,
            black_knights:   0x4200000000000000,
            black_bishops:   0x2400000000000000,
            black_rooks:     0x8100000000000000,
            black_queens:    0x0800000000000000,
            black_king:      0x1000000000000000,

            // all_white:       0x000000000000FFFF,
            // all_black:       0xFFFF000000000000,
            // all_pieces:      0xFFFF00000000FFFF,

            all_white:       0x00000000000000FF,
            all_black:       0xFF00000000000000,
            all_pieces:      0xFF000000000000FF,

            to_move:         1,

            white_check:     false,
            black_check:     false,

            white_checkmate: false,
            black_checkmate: false
        }
    }

    fn apply_move(&self, from: u64, to: u64) -> Option<Board> {
        
        let ib: Board = Board {
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
            to_move:       self.to_move ^ 1,
            white_check:     false,
            black_check:     false,

            white_checkmate: false,
            black_checkmate: false
        };
        
        let checks: (bool, bool) = self.check_check(&ib);

        // Check for illegally moving into check
        if (self.to_move == 1 && checks.0) || (self.to_move == 0 && checks.1) {
            return None
        }

        let mut wc: bool = false;
        let mut bc: bool = false;

        // Check for checkmates
        if (self.to_move == 1 && checks.1) || (self.to_move == 0 && checks.0) {
            if ib.generate_move_list().len() == 0 {
                if self.to_move == 1 {
                    bc = true;
                } else {
                    wc = true;
                }
            } 
        }

        let new_board: Board = Board {
            white_check:     checks.0,
            black_check:     checks.1,

            white_checkmate: wc,
            black_checkmate: bc,
            ..ib
        };

        Some(new_board)
    }

    fn check_check(&self, board: &Board) -> (bool, bool) {
        let white_to_move: u8 = 1;
        let black_to_move: u8 = 0;

        let mut white_checks: u64 = 0;
        let mut black_checks: u64 = 0;

        for piece_type in ['p', 'n', 'b', 'r', 'q'] {

            white_checks += board.reverse_move_mask(piece_type, black_to_move, board.white_king) & board.get_pieces(piece_type, black_to_move);
            black_checks += board.reverse_move_mask(piece_type, white_to_move, board.black_king) & board.get_pieces(piece_type, white_to_move);
        }

        (white_checks != 0, black_checks !=0)
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

    fn reverse_move_mask(&self, piece_type: char, to_move: u8, to: u64) -> u64 {

        let possible_pieces: u64 = self.get_pieces(piece_type, to_move);
        let own_pieces: u64;
        let enemy_pieces: u64;

        if to_move == 1 {
            own_pieces = self.all_white;
            enemy_pieces = self.all_black;
        } else {
            own_pieces = self.all_black;
            enemy_pieces = self.all_white;
        }

        if to & own_pieces != 0 {
            return 0;
        }

        if piece_type == 'p' {
            if to & enemy_pieces != 0 {
                // Diagonal capture (achieved by pawn capture mask colour flipped)
                return pawn_capture_mask(to, to_move ^ 1)
            } else {
                if to_move == 1 && get_bit_rf(to).0 == 3 {
                    // Possible double move
                    let mask: u64 = (to >> 8) | (to >> 16);
                    // Check for stacked pawns
                    if count_bits(mask & possible_pieces) == 2 {
                        return to >> 8;
                    }
                    
                    return mask;

                } else if to_move == 0 && get_bit_rf(to).0 == 4 {
                    // Possible double move
                    let mask: u64 = (to << 8) | (to << 16);
                    if count_bits(mask & possible_pieces) == 2 {
                        return to >> 8;
                    }
                    // Possible double move
                    return mask;
                }
                else {
                    if to_move == 1 {
                        return to >> 8;
                    } else {
                        return to << 8;
                    }
                }
            }
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

    pub fn parse_input(&self, input: &String) -> Option<Board> {

        if !input.is_ascii() {
            return None
        }

        let inlen = input.len();

        let target: String = input.clone().drain(inlen-2..).collect();
        let rank: u16 = target.chars().collect::<Vec<char>>()[1].to_digit(10)? as u16 - 1;
        let file: u16 = target.chars().collect::<Vec<char>>()[0].to_ascii_lowercase() as u16 - 97;

        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        let is_capture: bool = input.chars().collect::<Vec<char>>().contains(&'x');

        // Get piece type

        let first_char: char = input.chars().collect::<Vec<char>>()[0];
        let piece_type: char;

        if ['B', 'N', 'R', 'Q', 'K'].contains(&first_char) {
            piece_type = first_char.to_ascii_lowercase();
        } else if ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&first_char) {
            piece_type = 'p';
        } else {
            return None;
        }

        // Get possible disambiguation information

        let disambiguation: u64;
        if piece_type == 'p' && is_capture {

            let disambig_char = first_char;
            disambiguation = get_rank_or_file(disambig_char);
            if disambiguation == 0 {return None}

        } else if (inlen == 4 && !is_capture) || (inlen == 5 && is_capture) {

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
        } else {
            disambiguation = u64::MAX;
        }

        let available_pieces: u64 = self.get_pieces(piece_type, self.to_move) & disambiguation;

        let to: u64 = coord_to_bit((rank, file));
        let from: u64 = self.reverse_move_mask(piece_type, self.to_move, to) & available_pieces & disambiguation;

        if from == 0 {
            return None;
        } else if count_bits(from) > 1 {
            return None;
        } 

        self.apply_move(from, to)
    }

    pub fn generate_move_list(&self) -> Vec<Board> {

        let pawn_start_row: u64;
        let pawn_promote_row: u64;
        let own_pieces: u64;
        let enemy_pieces: u64;

        if self.to_move == 1 {
            // White to move
            pawn_start_row = RANK_2;
            pawn_promote_row = RANK_8;
            own_pieces = self.all_white;
            enemy_pieces = self.all_black;
        } else {
            // Black to move
            pawn_start_row = RANK_7;
            pawn_promote_row = RANK_1;
            own_pieces = self.all_black;
            enemy_pieces = self.all_white;
        }

        let mut move_list: Vec<Board> = vec![];

        // Generate pawn moves

        // Moves

        let pawns: Vec<u64> = iterate_over(self.get_pieces('p', self.to_move));

        for pawn in pawns {

            let mut pawn_moves: Vec<u64> = vec![];

            // Single moves

            let single_move: u64 = bidirectional_shift(pawn, 8, self.to_move);
        
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
                let double_move: u64 = bidirectional_shift(pawn, 16, self.to_move);
                if (self.all_pieces & double_move) == 0 {
                    pawn_moves.push(double_move)
                }
            }

            // Standard captures

            let capture_mask: u64 = pawn_capture_mask(pawn, self.to_move);
            for capture in iterate_over(capture_mask).iter() {
                if (capture & enemy_pieces) != 0 {
                    pawn_moves.push(*capture)
                }
            }

            // En passant not yet implemented
            
            for pawn_move in pawn_moves.iter() {

                let new_board = self.apply_move(pawn, *pawn_move);
                match new_board {
                    Some(board) => move_list.push(board),
                    None => {}
                }
            }

        }

        for piece_type in ['n', 'b', 'r', 'q', 'k'] {
            let pieces: Vec<u64> = iterate_over(self.get_pieces(piece_type, self.to_move));
            for piece in pieces {
                let move_mask = match piece_type {
                    'n' => knight_move_mask(piece, own_pieces),
                    'b' => bishop_move_mask(piece, own_pieces, enemy_pieces),
                    'r' => rook_move_mask(piece, own_pieces, enemy_pieces),
                    'q' => queen_move_mask(piece, own_pieces, enemy_pieces),
                    'k' => king_move_mask(piece, own_pieces),
                    _ => unreachable!()
                };
                for move_ in iterate_over(move_mask).iter() {
                    let new_board = self.apply_move(piece, *move_);
                    match new_board {
                        Some(board) => move_list.push(board),
                        None => {}
                    }
                }
            }
        }

        // Generate castling moves

        move_list

    }
}
