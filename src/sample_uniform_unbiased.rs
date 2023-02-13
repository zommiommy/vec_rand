use super::xorshift::xorshift;

/// Sample a random number in the range [0, number)
///
/// Based on [nearly divisionless by Daniel Lamire](https://lemire-me.cdn.ampproject.org/v/s/lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/?amp=&usqp=mq331AQFKAGwASA=&amp_js_v=0.1#aoh=16030463837713&csi=1&referrer=https://www.google.com&amp_tf=From %1$s&ampshare=https://lemire.me/blog/2019/06/06/nearly-divisionless-random-integer-generation-on-various-systems/).
#[inline]
pub fn sample_uniform_unbiased_nearly_divisionless(number: u64, mut seed: u64) -> usize {
    seed = xorshift(seed);
    let mut m = seed as u128 * number as u128;
    let mut l = m as u64;

    if l < number {
        let t: u64 = (0 - number) % number;
        while l < t {
            seed = xorshift(seed);
            m = seed as u128 * number as u128;
            l = m as u64;
        }
    }

    return (m >> 64) as usize;
}

/// Sample a random number in the range [0, number)
///
#[inline]
pub fn sample_uniform_unbiased_simple(number: u64, mut seed: u64) -> usize {
    if number == 1 {
        return 0;
    }
    let limit = u64::MAX - (u64::MAX % number);
    loop {
        seed = xorshift(seed);
        if seed < limit {
            return (seed % number) as usize;
        }
    }
}
