
#[inline(always)]
/// Generate a random u64 using the by-the-book implementation
/// 
/// Example:
/// 
/// ```
///  let mut seed: u64 = 0xBAD5EEDdeadbeef;
/// let value = xorshift(& mut seed);
/// println!("{:?}", value);
/// ```
pub fn xorshift_plain(seed: & mut u64) -> u64 {
    *seed ^= *seed << 17;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 13;
    *seed
}