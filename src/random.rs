use xorshift::xorshift;
use xorshiro256plus::xorshiro256plus;

static mut GLOBAL_SEED: [u64; 4] = [
    6591408588322595484,
    5451729388608518856,
    8913376598984957243,
    17912695770704705270,
];

#[inline(always)]
/// Return a random u64.
///
/// # Implementation details
/// The implementations is based on xorshiro256+ which seems to be the fastest floating point generator.
/// The reference implementation can be found [here](http://prng.di.unimi.it/xoshiro256plus.c).
/// Xorshiro256+ generate a  random u64 so we need to convert it to f64.
///
/// One important detail about xorshiro256+ is that it has low entropy in the lower 3 bits.
///
/// One possible optimization might be to generate several random values in parallel exploiting
/// AVX / SSE instructions and then use these values. An implemnetation could be found [here](http://prng.di.unimi.it/xoshiro256+-vect-speed.c)
///
/// # Examples
/// ```
/// use vec_rand::random_u64;
///
/// let rnd: u64 = random_u64();
/// println!("The random value is: {}", rnd);
/// ```
pub fn random_u64() -> u64 {
    unsafe { xorshiro256plus(&mut GLOBAL_SEED) }
}

#[inline(always)]
/// Return a random f64 between 0 and 1.
///
/// # Implementation details
/// We generate a pseudo-random number using xorshiro256+ and then we convert it to a float.`
///
/// One important detail about xorshiro256+ is that it has low entropy in the lower bits.
/// This is not a problem since we generate 64bits but we will only need 53.
///
/// There are two main methods to convert from u64 to f64 and they can be found [here](http://prng.di.unimi.it/)
///
///
/// Basically we are setupping the exponent and mantissa of the float and then punning the value to a float
///
/// The "simplest" is to multiply the value for the right exponent:alloc
/// ```
/// # use vec_rand::random_u64;
/// let result: f64 = (random_u64() >> 11) as f64 * 2.0f64.powf(-53.0);
/// ```
///
/// There is also a second way that exploit type punning:
///
/// ```
/// # use vec_rand::random_u64;
/// let v: u64 = (random_u64() >> 11) | (1023 << 52);
/// let r: f64 = unsafe{f64::from_le_bytes(v.to_le_bytes())};
/// let result: f64 = r - 1f64;
/// ```
/// the informations about the structure of a f64 was taken from [IEEE 754](https://standards.ieee.org/content/ieee-standards/en/standard/754-2019.html)
///
/// First we shift the value in order to fit the high-entropy values in the mantissa of the float.
///
/// Then we se the bits from 1 to 12 to 1023 so that we set the exponent to 1.
/// (Since the computed exponent is e - 1022 where e is the value we set)
///
/// Then we convert this u64 to a random f64 from 1 to 2.
///
/// The type punning is made with:
/// ```
/// # let v: u64 = 100;
/// let r: f64 = unsafe{f64::from_le_bytes(v.to_le_bytes())};
/// ```
/// The C equivalent is:
/// ```C
/// double r = *((double *)&v);
/// ```
///
/// The last step is to fix the range form 1 - 2, to 0 - 1.
///
/// As Vigna [says](http://prng.di.unimi.it/), these two methods should have equivalent performances on modern hardware.
/// But in our benchmarks we found the second (and more complicated) one to be slightly faster.
///
/// # Examples
/// ```
/// use vec_rand::random_f64;
///
/// let frnd: f64 = random_f64(0x5eed);
/// assert!(0.0 <= frnd && frnd <= 1.0);
/// println!("The random value is: {}", frnd);
/// ```
pub fn random_f64(seed: u64) -> f64 {
    let rnd = xorshift(xorshift(seed ^ 10) ^ 1337);
    let v: u64 = (rnd >> 11) | (1023 << 52);
    let r: f64 = f64::from_le_bytes(v.to_le_bytes());
    r - 1f64
}

#[inline(always)]
/// Return a random f32 between 0 and 1.
///
/// # Examples
/// ```
/// use vec_rand::random_f32;
///
/// let frnd: f32 = random_f32(0x5eed);
/// println!("The random value is: {}", frnd);
/// assert!(0.0 <= frnd && frnd <= 1.0);
/// ```
pub fn random_f32(seed: u64) -> f32 {
    let rnd = xorshift(xorshift(seed ^ 10) ^ 1337);
    println!("VALORE: {}", rnd);
    let v: u32 = (((rnd >> 8) & 0xffffff) | (127 << 23)) as u32;
    let r: f32 = f32::from_le_bytes(v.to_le_bytes());
    r - 1f32
}
