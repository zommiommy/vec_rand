# vec_rand [![Build Status](https://travis-ci.org/zommiommy/vec_rand.svg?branch=master)](https://travis-ci.org/zommiommy/vec_rand) ![](https://img.shields.io/badge/platform-linux--64%20%7C%20osx--64%20%7C%20win--64-lightgrey)

Rust crate where we implement vectorized versions of fast pseudo-random number generators.

These implementations are in no way ment to be Cryptographically safe, Their intended porpouse is to do MonteCarlo simulations and Random-Walks on graphs.

This repository will collect several implementations of various algorithm to explore in a systematic way which method is faster.

The implementations are meant for `x86_64` processors with `avx2`.

The main prng in this repo are the vectorized versions of Xorshift such as:

```rust

pub fn xorshift_avx(seed: & mut [u64; 4]) -> [u64; 4] {
    let mut result: [u64; 4] = [0; 4];
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rsi]\n",
            // << 13
            "vpsllq ymm1, ymm0, 13\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // << 17
            "vpsllq ymm1, ymm0, 17\n",
            // ^c
            "vpxor ymm0, ymm0, ymm1\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0\n",
            "vmovdqu ymmword ptr [rsi], ymm0\n"
        ),
        inout("rsi") seed => _,
        inout("rdi") result.as_mut_ptr() => _,
        );
    }
    result
}
```

# Usage

By default this repo targets the current cpu using `-C target-cpu=native`.

This can be changed by overriding the env_var:
`export RUSTFLAGS='-C target-cpu=skylake'`

or just prefixing the command with it:

```
RUSTFLAGS='-C target-cpu=skylake' cargo test
```

```
RUSTFLAGS='-C target-cpu=skylake' cargo run --release
```

```
RUSTFLAGS='-C target-cpu=skylake' cargo bench
```

Or by changing the `.cargo/config` with:

```
[build]
rustflags = ["-C", "target-cpu=skylake"]
```

# Benchmarks

To run the benchmarks, [once you have rust nightly installed](https://rustup.rs/) (nightly is needed to use inline assembly), just clone the repo and run `RUSTFLAGS='-C target-cpu=native' cargo bench`

On my `Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz` I get the following timings:

### xorshiro256plus

The test is to generate 32_000 random u64. `thread_rng` is the default rust rand implementation.

```
test test_thread_rng              ... bench:     390,214 ns/iter (+/- 9,748)
test test_xorshiro256plus         ... bench:      23,801 ns/iter (+/- 2,429)
test test_xorshiro256plus_avx     ... bench:      23,042 ns/iter (+/- 2,809)
test test_xorshiro256plus_avx_ss4 ... bench:      15,732 ns/iter (+/- 1,247)
```

### xorshift

The test is to generate 32_000 random u64. `thread_rng` is the default rust rand implementation.

```
test test_thread_rng              ... bench:     386,575 ns/iter (+/- 18,322)
test test_xorshift                ... bench:      51,366 ns/iter (+/- 2,220)
test test_xorshift_avx            ... bench:      26,689 ns/iter (+/- 2,493)
test test_xorshift_avx_intrinsics ... bench:     309,970 ns/iter (+/- 28,109)
test test_xorshift_avx_ss4        ... bench:      19,482 ns/iter (+/- 1,006)
test test_xorshift_avx_ss8        ... bench:       8,632 ns/iter (+/- 512)
```

The best time is the one of `xorshif_avx_ss4` which achieves `8,632 ns` for ` 32_000 random bytes` which is equals to `~270ps` for each u64.
This corrisponds to approximately to ~`27 Gib/s`.

### splitmix64

The test is to generate 32000 random u64. `thread_rng` is the default rust rand implementation.

```
test test_splitmix64 ... bench:     107,781 ns/iter (+/- 11,842)
test test_thread_rng ... bench:     391,169 ns/iter (+/- 15,654)
```

### generate a random vector of `u64`

The test is to generate a vector of 1_000_000 u64s.

```
test test_gen_random_vec_1        ... bench:   1,686,786 ns/iter (+/- 27,714)
test test_gen_random_vec_32_1     ... bench:   1,723,869 ns/iter (+/- 67,809)
test test_gen_random_vec_32_4_1   ... bench:   3,062,667 ns/iter (+/- 136,449)
test test_gen_random_vec_4_1      ... bench:   1,545,974 ns/iter (+/- 63,395)
test test_gen_range_of_thread_rng ... bench:   8,768,966 ns/iter (+/- 205,310)
test test_with_xorshift           ... bench:   2,683,356 ns/iter (+/- 241,193)
test test_with_xorshiro256plus    ... bench:   2,374,508 ns/iter (+/- 105,906)
```

The tests with name `test_gen_random_vec(_\d+)` uses group by filling, meaning that `test_gen_random_vec_32_4_1` will first fill the vector with batches of 32 u64s using `xorshift_avx_ss8` then in the remaining values will be filled with batches of 4 u64s using `xorshift_avx`, finally, any remaining values will be filled singuarly with `xorshift`.

### cumulative sums for `f64`

The test is to compute the cumulative sum for 10_000 values.

```
test test_cumsum_f64                ... bench:      24,316 ns/iter (+/- 1,643)
test test_cumsum_f64_avx_intrinsics ... bench:     139,113 ns/iter (+/- 8,388)
test test_cumsum_f64_scan           ... bench:      29,102 ns/iter (+/- 609)
test test_cumsum_f64_sse_intrinsics ... bench:       9,278 ns/iter (+/- 245)
test test_cumsum_f64_unrolled       ... bench:      11,569 ns/iter (+/- 861)
```

### cumulative sums for `f32`

The test is to compute the cumulative sum for 10_000 values.

```
test test_cumsum_f32                ... bench:      24,086 ns/iter (+/- 787)
test test_cumsum_f32_scan           ... bench:      27,565 ns/iter (+/- 2,567)
test test_cumsum_f32_sse_intrinsics ... bench:       4,040 ns/iter (+/- 152)
test test_cumsum_f32_unrolled       ... bench:      11,613 ns/iter (+/- 914)
```

### Sampling

The test is to extract an index from a vector with 100_000 "weights" f64.

```
test test_sample                ... bench:     107,844 ns/iter (+/- 18,400)
test test_sample_avx            ... bench:      84,182 ns/iter (+/- 4,458)
test test_weighted_index_sample ... bench:     244,001 ns/iter (+/- 34,833)
```

# Throughtput analysis

The results on my `Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz`:

```
Measuring mean number of cycles per random u64


xorshift

mean cycles: 4.443891164        alg: xorshift
mean cycles: 2.5617648905       alg: xorshift_avx
mean cycles: 1.702883781375     alg: xorshift_avx_ss4
mean cycles: 0.935907395375     alg: xorshift_avx_ss8


xorshiro256plus

mean cycles: 2.249175442        alg: xorshiro256plus
mean cycles: 2.2306521235       alg: xorshiro256plus_avx
mean cycles: 1.834847551875     alg: xorshiro256plus_avx_ss4
```

These measurements are made with:

```rust
let start: u64 = rdtsc();
for _ in 0..SIZE {
   algorithm(& mut seed);
}
let v = (rdtsc() - start) as f64 / SIZE as f64 / batch_size;
```

where `batch_size` is how may u64 the algorithm generate for each call.
