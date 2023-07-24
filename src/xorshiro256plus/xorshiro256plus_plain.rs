#[inline(always)]
/// Generate a random u64 by using xorshift256plus with the [reference implementation](http://prng.di.unimi.it/xoshiro256plus.c) translated to rust.
///
/// Example:
///
/// ```
/// use vec_rand::*;
///  let mut seed: [u64; 4] = [
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///  ];
/// let value = xorshiro256plus(& mut seed);
/// println!("{:?}", value);
/// ```
pub fn xorshiro256plus_plain(s: &mut [u64; 4]) -> u64 {
    let (result, _): (u64, bool) = s[0].overflowing_add(s[3]);
    let t: u64 = s[1] << 17;
    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];

    s[2] ^= t;

    s[3] = s[3].rotate_left(45);

    result
}
