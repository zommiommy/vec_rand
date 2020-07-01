
/// the suggested method to initialize the seed of xorshiro
/// http://prng.di.unimi.it/splitmix64.c 
pub fn splitmix64(x: u64) -> u64 {
	let (mut z, _): (u64, bool) = x.overflowing_add(0x9e3779b97f4a7c15);
	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
	z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
	return z ^ (z >> 31);
}


/// method suggested here
/// http://prng.di.unimi.it/
/// We suggest to use a SplitMix64 to initialize the state of our generators
/// starting from a 64-bit seed, as research has shown that initialization 
/// must be performed with a generator radically different in nature from 
/// the one initialized to avoid correlation on similar seeds.
pub fn initialize_seed(start_seed: u64, size: usize) -> Vec<u64> {
    let mut result = Vec::with_capacity(size);
    let mut se = start_seed;
    for _ in 0..size {
        result.push(splitmix64(se));
        se = se.wrapping_add(0x9e3779b97f4a7c15);
    }

    result
}
