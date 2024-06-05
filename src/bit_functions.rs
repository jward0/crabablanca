use crate::constants::*;

pub fn get_lsb(bits: u64) -> u64 {
    // assert_ne!(bits, 0);

    if bits == 0 {
        return 0
    }

    1 << bits.trailing_zeros() as u64
}

pub fn get_msb(bits: u64) -> u64 {
    // assert_ne!(bits, 0);

    if bits == 0 {
        return 0
    }
    1 << 63 - bits.leading_zeros() as u64
}

pub fn count_bits(bits: u64) -> u8 {
    iterate_over(bits).len() as u8
}

pub fn get_rank_or_file(c: char) -> u64 {

    if ['1', '2', '3', '4', '5', '6', '7', '8'].contains(&c) {

        let rank = c.to_digit(10)?;
        return RANK_1 << rank - 1;

    } else if ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'].contains(&c){

        let file = c.to_ascii_lowercase() as u32 - 61;
        return FILE_A << file - 1;

    } else {
        return 0;
    }
}

pub fn bidirectional_shift(bits: u64, shift: u64, direction: u8) -> u64 {
    // direction is 1 for white move, 0 for black move
    if direction == 1 {
        bits << shift
    } else {
        bits >> shift
    }
}

fn i8_shift(bits: u64, shift: i8) -> u64 {
    if shift > 0 {
        return bits << shift as u64
    } else {
        return bits >> (-1*shift) as u64
    }
}

pub fn fill_from(bit: u64) -> u64 {

    if bit == 0 {
        return 0
    }

    u64::MAX << bit.trailing_zeros() as u64
}

pub fn fill_to(bit: u64) -> u64 {

    if bit == 0 {
        return 0
    }

    u64::MAX >> 63 - bit.trailing_zeros() as u64
}

pub fn iterate_over(bits: u64) -> Vec<u64> {
    let mut locs: Vec<u64> = vec![];
    let mut tmp = bits;

    while tmp > 0 {
        let lsb: u64 = get_lsb(tmp);
        locs.push(lsb);
        tmp -= lsb;
    }

    locs
}

pub fn bit_to_coord(bit: u64) -> (u16, u16) {
    assert_ne!(bit, 0);
    let pos = bit.trailing_zeros() as u16;
    (pos % 8, pos / 8)
}

pub fn coord_to_bit(coords: (u16, u16)) -> u64 {
    (1 << coords.0) << 8*coords.1
}

pub fn move_piece(bits: u64, from: u64, to: u64) -> u64 {
    if (bits & from) != 0 {
        // This is the bitboard the piece belongs to
        // So move bit
        (bits & !from) | to
    } else {
        // This is not the bitboard the piece belongs to
        // So remove bit
        bits & !to
    }
}

pub fn get_bit_rf(bit: u64) -> (u8, u8) {
    // Returns as (rank, file) - note that these are zero-indexed
    ((bit.trailing_zeros() / 8) as u8, (bit.trailing_zeros() % 8) as u8)
}

pub fn pawn_capture_mask(bit: u64, to_move: u8) -> u64 {

    if to_move == 1 { // White pawn capture mask
        if (bit & FILE_A) != 0 {
            bit << 9
        } else if (bit & FILE_H) != 0 {
            bit << 7
        } else {
            (bit << 7) + (bit << 9)
        }
    } else { // Black pawn capture mask
        if (bit & FILE_A) != 0 {
            bit >> 7
        } else if (bit & FILE_H) != 0 {
            bit >> 9
        } else {
            (bit >> 7) + (bit >> 9)
        }
    }
}

pub fn sl(bit: u64) -> u64 {
    if bit & FILE_A != 0 {0} else {bit >> 1}
}

pub fn sr(bit: u64) -> u64 {
    if bit & FILE_H != 0 {0} else {bit << 1}
}

pub fn su(bit: u64) -> u64 {
    if bit & RANK_8 != 0 {0} else {bit << 8}
}

pub fn sd(bit: u64) -> u64 {
    if bit & RANK_1 != 0 {0} else {bit >> 8}
}

pub fn knight_move_mask(bit: u64, own_pieces: u64) -> u64 {

    let mut mask: u64 = 0;
    let mut shifts: Vec<i8> = vec![15, 17, 10, -6, -15, -17, -10, 6];

    if (bit & FILE_A) != 0 {
        shifts[0] = 0;
        shifts[5] = 0;
        shifts[6] = 0;
        shifts[7] = 0;
    }
    if (bit & FILE_B) != 0 {
        shifts[6] = 0;
        shifts[7] = 0;
    }
    if (bit & FILE_G) != 0 {
        shifts[2] = 0;
        shifts[3] = 0;
    }
    if (bit & FILE_H) != 0 {
        shifts[1] = 0;
        shifts[2] = 0;
        shifts[3] = 0;
        shifts[4] = 0;
    }
    if (bit & RANK_1) != 0 {
        shifts[3] = 0;
        shifts[4] = 0;
        shifts[5] = 0;
        shifts[6] = 0;
    }
    if (bit & RANK_2) != 0 {
        shifts[4] = 0;
        shifts[5] = 0;
    }
    if (bit & RANK_7) != 0 {
        shifts[1] = 0;
        shifts[2] = 0;
    }
    if (bit & RANK_8) != 0 {
        shifts[0] = 0;
        shifts[1] = 0;
        shifts[2] = 0;
        shifts[7] = 0;
    }

    for shift in shifts.iter() {
        if *shift != 0 {
            // mask += ((bit as i8) + shift) as u64;
            mask += i8_shift(bit, *shift)
        }
    }

    mask & !own_pieces & !bit
}

pub fn bishop_move_mask(bit: u64, own_pieces: u64, enemy_pieces: u64) -> u64 {
    // TODO: tidy this up a bit (make more bitwise)
    let (r, f) = get_bit_rf(bit);

    let main_diag: u64;
    let anti_diag: u64;

    if r >= f {
        main_diag = MAIN_DIAG << (r - f) * 8;
    } else {
        main_diag = MAIN_DIAG >> (f - r) * 8;
    }

    if r + f >= 7 {
        anti_diag = ANTI_DIAG << (r + f - 7) * 8;
    } else {
        anti_diag = ANTI_DIAG >> (7 - r - f) * 8;
    }

    let main_ray: u64 = block_ray(bit, main_diag, own_pieces, enemy_pieces);
    let anti_ray: u64 = block_ray(bit, anti_diag, own_pieces, enemy_pieces);

    (main_ray | anti_ray) & !bit

    // (main_diag | anti_diag) & !bit
}

pub fn rook_move_mask(bit: u64, own_pieces: u64, enemy_pieces: u64) -> u64 {

    let (r, f) = get_bit_rf(bit);

    let hori_ray: u64 = block_ray(bit, RANK_1 << 8 * r, own_pieces, enemy_pieces);
    let vert_ray: u64 = block_ray(bit, FILE_A << f, own_pieces, enemy_pieces);

    (hori_ray | vert_ray) & !bit 

    // ((RANK_1 << 8 * r) | (FILE_A << f)) & !bit
}

pub fn queen_move_mask(bit: u64, own_pieces: u64, enemy_pieces: u64) -> u64 {
    rook_move_mask(bit, own_pieces, enemy_pieces) | bishop_move_mask(bit, own_pieces, enemy_pieces)
}

pub fn block_ray(bit: u64, ray: u64, own_pieces: u64, enemy_pieces: u64) -> u64 {

    let forward: u64 = fill_from(bit);
    let backward: u64 = fill_to(bit);

    let forward_ray: u64 = ray & forward;
    let backward_ray: u64 = ray & backward;

    let own_forward_blocker: u64 = fill_from(get_lsb((own_pieces & !bit) & forward & forward_ray));
    let own_backward_blocker: u64 = fill_to(get_msb((own_pieces & !bit) & backward & backward_ray));

    let enemy_forward_blocker: u64 = fill_from(get_lsb(enemy_pieces & forward & forward_ray)) << 1;
    let enemy_backward_blocker: u64 = fill_to(get_msb(enemy_pieces & backward & backward_ray)) >> 1;

    (forward_ray & !(enemy_forward_blocker | own_forward_blocker)) | (backward_ray & !(enemy_backward_blocker | own_backward_blocker))
}

pub fn king_move_mask(bit: u64, own_pieces: u64) -> u64 {

    let row: u64 = sl(bit) | sr(bit) | bit;
    (row | su(row) | sd(row) & !bit) & !own_pieces
}