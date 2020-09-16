// bit ops


pub fn or (a: u64, b: u64) -> u64 {
  a | b
}

pub fn and (a: u64, b: u64) -> u64 {
  a & b
}

pub fn xor (a: u64, b: u64) -> u64 {
  a ^ b
}

pub fn not (a: u64) -> u64 {
  !a
}

pub fn left_shift (a: u64, shift: u64) -> u64 {
  a << shift
}

pub fn right_shift (a: u64, shift: u64) -> u64 {
  a >> shift
}

pub fn all_one () -> u64 {
  not(0)
}

pub fn self_xor (a: u64) -> u64 {
  xor(a, all_one())
}

pub fn make_right_zero (a: u64, right_bits: u64) -> u64 {
  and(a, left_shift(all_one(), right_bits))
}

pub fn make_left_zero (a: u64, left_bits: u64) -> u64 {
  and(a, right_shift(all_one(), left_bits))
}

pub fn n_bit_value (a: u64, n: u64) -> u64 {
  and(right_shift(a, n), 1)
}

// 0b1101 e.g.: n_bit_power(0b1101u, 1) ==> 0 * 2^1 = 0
pub fn n_bit_power (a: u64, n: u64) -> u64 {
  and(a, left_shift(1, n-1))
}

pub fn is_odd (a: u64) -> bool {
  and(a, 1)
}

// 从低位开始算起的第一个1清0
pub fn clear_lsb (a: u64) -> u64 {
  and(a, a-1);
}

// 得到从低位开始的第一个1所表示的数值
pub fn get_lsb_value (a: u64) -> u64 {
  and(a, -a)
}


