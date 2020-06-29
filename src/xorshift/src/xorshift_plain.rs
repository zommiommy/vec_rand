
#[inline(always)]
pub fn xorshift_plain(seed: & mut u64) -> u64 {
    for _ in 0..2 {
        *seed ^= *seed << 17;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 13;
    }
    *seed
}