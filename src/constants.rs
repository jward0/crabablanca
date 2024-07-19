pub const RANK_1: u64 = 0x00000000000000FF;
pub const RANK_2: u64 = RANK_1 << 8;
pub const RANK_7: u64 = RANK_1 << 48;
pub const RANK_8: u64 = RANK_1 << 56;
// pub const RANK_2: u64 = 0x000000000000FF00;
// pub const RANK_7: u64 = 0x00FF000000000000;
// pub const RANK_8: u64 = 0xFF00000000000000;

pub const FILE_A: u64 = 0x0101010101010101;
pub const FILE_B: u64 = FILE_A << 1;
pub const FILE_G: u64 = FILE_A << 6;
pub const FILE_H: u64 = FILE_A << 7;
// pub const FILE_H: u64 = 0x8080808080808080;


pub const MAIN_DIAG: u64 = 0x8040201008040201;
pub const ANTI_DIAG: u64 = 0x0102040810204080;

pub const CENTRE: u64 = 0x0000001818000000;