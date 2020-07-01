
// the suggested method to initialize the seed of xorshiro
fn splitmix64(x: u64) -> u64 {
    // http://prng.di.unimi.it/splitmix64.c
	let (mut z, _): (u64, bool) = x.overflowing_add(0x9e3779b97f4a7c15);
	z = (z ^ (z >> 30)) * 0xbf58476d1ce4e5b9;
	z = (z ^ (z >> 27)) * 0x94d049bb133111eb;
	return z ^ (z >> 31);
}

pub fn initialize_seed(start_seed: u64) {
    // method suggested here
    // http://prng.di.unimi.it/
    // We suggest to use a SplitMix64 to initialize the state of our generators
    // starting from a 64-bit seed, as research has shown that initialization 
    // must be performed with a generator radically different in nature from 
    // the one initialized to avoid correlation on similar seeds.
    let mut se = start_seed;
    println!("{:?}", splitmix64(se));
    se += 0x9e3779b97f4a7c15;
    println!("{:?}", splitmix64(se));
    se += 0x9e3779b97f4a7c15;
    println!("{:?}", splitmix64(se));
    se += 0x9e3779b97f4a7c15;
    println!("{:?}", splitmix64(se));
}
