
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m256i,
    // << 
    _mm256_slli_epi64,
    // >>
    _mm256_srli_epi64,
    // ^ 
    _mm256_xor_si256,
    // mem -> reg
    _mm256_loadu_si256,
    // reg -> mem
    _mm256_storeu_si256

};

#[inline(always)]
pub fn xorshift_sse_intrinsics(seed: & mut [u64; 4]) -> [u64; 4] {
    let mut result: [u64; 4] = [1337, 0xdeadbeef, 0xc0febabe, 0xbad5eed];
    unsafe{
        let mut temp: __m256i;
        let mut seed: __m256i = _mm256_loadu_si256(seed.as_mut_ptr() as *mut __m256i);
        temp = _mm256_slli_epi64(seed, 13);
        seed = _mm256_xor_si256(seed, temp);
        temp = _mm256_srli_epi64(seed, 7);
        seed = _mm256_xor_si256(seed, temp);
        temp = _mm256_slli_epi64(seed, 17);
        seed = _mm256_xor_si256(seed, temp);
        _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, seed);
    }
    result
}