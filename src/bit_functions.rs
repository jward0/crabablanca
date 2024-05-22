use crate::constants::*;

pub fn get_lsb(bits: u64) -> u64 {
    assert_ne!(bits, 0);
    1 << bits.trailing_zeros() as u64
}

pub fn get_msb(bits: u64) -> u64 {
    assert_ne!(bits, 0);
    1 << 63 - bits.leading_zeros() as u64
}

pub fn bidirectional_shift(bits: u64, shift: u64, direction: u8) -> u64 {
    // direction is 1 for white move, 0 for black move
    if direction == 1 {
        bits << shift
    } else {
        bits >> shift
    }
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

pub fn apply_move(bits: u64, from: u64, to: u64) -> u64 {
    if (bits & from) == 1 {
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

pub fn pawn_capture_mask(bit: u64) -> u64 {
    if (bit & FILE_A) == 1 {
        bit << 9
    } else if (bit & FILE_H) == 1 {
        bit << 7
    } else {
        bit << 7 + bit << 9
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

pub fn knight_move_mask(bit: u64) -> u64 {

    let mut mask: u64 = 0;
    let mut shifts: Vec<i8> = vec![15, 17, 10, -6, -15, -17, -10, 6];

    if (bit & FILE_A) == 1 {
        shifts[0] = 0;
        shifts[5] = 0;
        shifts[6] = 0;
        shifts[7] = 0;
    }
    if (bit & FILE_B) == 1 {
        shifts[6] = 0;
        shifts[7] = 0;
    }
    if (bit & FILE_G) == 1 {
        shifts[2] = 0;
        shifts[3] = 0;
    }
    if (bit & FILE_H) == 1 {
        shifts[1] = 0;
        shifts[2] = 0;
        shifts[3] = 0;
        shifts[4] = 0;
    }
    if (bit & RANK_1) == 1 {
        shifts[3] = 0;
        shifts[4] = 0;
        shifts[5] = 0;
        shifts[6] = 0;
    }
    if (bit & RANK_2) == 1 {
        shifts[4] = 0;
        shifts[5] = 0;
    }
    if (bit & RANK_7) == 1 {
        shifts[1] = 0;
        shifts[2] = 0;
    }
    if (bit & RANK_8) == 1 {
        shifts[0] = 0;
        shifts[1] = 0;
        shifts[2] = 0;
        shifts[7] = 0;
    }

    for shift in shifts.iter() {
        if *shift != 0 {
            mask += ((mask as i8) + shift) as u64;
        }
    }

    mask
}

pub fn bishop_move_mask(bit: u64) -> u64 {
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

    (main_diag | anti_diag) & !bit
}

pub fn rook_move_mask(bit: u64) -> u64 {
    let (r, f) = get_bit_rf(bit);
    ((RANK_1 << 8 * r) | (FILE_A << f)) & !bit
}

pub fn queen_move_mask(bit: u64) -> u64 {
    rook_move_mask(bit) | bishop_move_mask(bit)
}

pub fn king_move_mask(bit: u64) -> u64 {

    let row: u64 = sl(bit) & sr(bit) & bit;
    row & su(row) & sd(row) & !bit
}