
#[inline(always)]
/// Generate a random u64 using the by-the-book implementation
/// 
/// Example:
/// 
/// ```
/// let value = xorshift(0xBAD5EEDdeadbeef);
/// println!("{:?}", value);
/// ```
pub fn xorshift_plain(seed: u64) -> u64 {
    let mut s = seed;
    s ^= s << 13;
    s ^= s >> 7;
    s ^= s << 17;
    s
}