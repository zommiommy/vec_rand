#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m128d,
    // sum two vector of f64
    _mm_add_pd,
    // cast __m128d to __m128di
    // see _mm_castsi128_ps
    _mm_castpd_si128,
    // cast __m128di  to __m128d
    // it's only for compilation, it does not gen instructions
    _mm_castsi128_pd,
    // Memory -> Vec but slower
    _mm_loadu_pd,
    // set vec to zero
    _mm_setzero_pd,
    // Shiffle the vecotr according to the mask given
    _mm_shuffle_pd,
    // shift vector left and insert zeros
    _mm_slli_si128,
    // Vec -> Memory but slower
    _mm_storeu_pd,
};

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn scan_sse(mut x: __m128d) -> __m128d {
    //
    // pass:
    //      f2, f1 +
    //      f1,  0 =
    //      f21, f1
    //
    // -> Meh, 1 add + 1 shift instead of 1 add, not great
    unsafe {
        x = _mm_add_pd(x, _mm_castsi128_pd(_mm_slli_si128(_mm_castpd_si128(x), 8)));
    }
    x
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn cumsum_f64_sse_intrinsics(random_vec: &mut [f64]) {
    if random_vec.len() <= 1 {
        return;
    }
    let max = (random_vec.len() >> 1) << 1;
    unsafe {
        let mut offset: __m128d = _mm_setzero_pd();
        for i in (0..max).step_by(2) {
            // it should be __mm_load_ps but if the values are not aligned it
            // raises a seg-fault so we use the slower _mm_loadu_ps until we figure
            // out how to ensure the alignmenet of the vector
            // loat the 4 values
            let x: __m128d = _mm_loadu_pd(random_vec.as_ptr().wrapping_add(i));
            // compute the local cumulative sum
            let mut out: __m128d = scan_sse(x);
            // add the local cumulative sum to the current offset
            out = _mm_add_pd(out, offset);
            // get the internal floats array of the result vec
            let ptr: *mut f64 = random_vec.as_mut_ptr();
            // store the value in the vector
            _mm_storeu_pd(ptr.add(i), out);
            // Update the current offset (aka the last value of out)
            offset = _mm_shuffle_pd(out, out, 3);
        }
    }

    if random_vec.len() % 2 == 1 {
        random_vec[max] += random_vec[max - 1];
    };
}
