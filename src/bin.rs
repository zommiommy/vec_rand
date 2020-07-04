#![feature(asm)]
use log::{debug};
use core::arch::x86_64::{_rdtsc, __rdtscp};
use vec_rand;

fn rdtsc() -> u64{
    let mut x: u32 = 0;
    // __rdtscp it's the serialized version of _rdtsc
    // this should give us more consistent results
    unsafe{
        __rdtscp(& mut x)
    }
}

const SIZE: usize = 1_000_000_000;


fn test_xorshift() -> u64{
    let mut seed: u64 = 0xBAD5EEDdeadbeef;
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        seed = vec_rand::xorshift::xorshift(seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64;
    println!("mean cycles: {}\talg: xorshift", v);
    seed
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn test_xorshift_avx() -> [u64; 4]{
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshift::xorshift_avx(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64 / 4.0;
    println!("mean cycles: {}\talg: xorshift_avx", v);
    seed
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn test_xorshift_avx_ss4() -> [u64; 16] {
    let mut seed: [u64; 16] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshift::xorshift_avx_ss4(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64 / 16.0;
    println!("mean cycles: {}\talg: xorshift_avx_ss4", v);
    seed
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn test_xorshift_avx_ss8() -> [u64; 32] {
    let mut seed: [u64; 32] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshift::xorshift_avx_ss8(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64 / 32.0;
    println!("mean cycles: {}\talg: xorshift_avx_ss8", v);
    seed
}

fn test_xorshiro256plus() -> [u64; 4]{
    let mut seed: [u64; 4] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshiro256plus::xorshiro256plus(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64;
    println!("mean cycles: {}\talg: xorshiro256plus", v);
    seed
}


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn test_xorshiro256plus_avx() -> [u64; 16]{
    let mut seed: [u64; 16] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshiro256plus::xorshiro256plus_avx(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64 / 4.0;
    println!("mean cycles: {}\talg: xorshiro256plus_avx", v);
    seed
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
fn test_xorshiro256plus_avx_ss4() -> [u64; 64]{
    let mut seed: [u64; 64] = [
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
        0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
    ];
    let start: u64 = rdtsc();
    for _ in 0..SIZE {
        vec_rand::xorshiro256plus::xorshiro256plus_avx_ss4(& mut seed);
    }
    let v = (rdtsc() - start) as f64 / SIZE as f64 / 16.0;

    println!("mean cycles: {}\talg: xorshiro256plus_avx_ss4", v);
    seed
}

fn main() {
    println!("Measuring mean number of cycles per random u64\n");
    println!("\nxorshift\n");

    // using debug to introduce dependancies on the result of the seed so that
    // the compiler don't optimize away the algorithms

    let result = test_xorshift();
    debug!("{}", result);

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    let result = test_xorshift_avx();
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    debug!("{:?}", result);

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    let result = test_xorshift_avx_ss4();
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    debug!("{:?}", result);

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    let result = test_xorshift_avx_ss8();
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    for i in result.iter() {
        debug!("{:?}", i);
    }
    
    println!("\n\nxorshiro256plus\n");

    let result = test_xorshiro256plus();
    debug!("{:?}", result);

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    let result = test_xorshiro256plus_avx();
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    debug!("{:?}", result);

    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    let result = test_xorshiro256plus_avx_ss4();
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
    for i in result.iter() {
        debug!("{:?}", i);
    }
}
