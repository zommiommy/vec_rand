#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m256d,
    // sum two vector of f64
    _mm256_add_pd,
    // cast __m256d to __m256di
    // see _mm_castsi128_ps
    _mm256_castpd_si256,
    // cast __m256di  to __m256d
    // it's only for compilation, it does not gen instructions
    _mm256_castsi256_pd,
    // Memory -> Vec but slower
    _mm256_loadu_pd,
    // Shiffle the vecotr according to the mask given
    _mm256_permute2f128_pd,
    _mm256_permute_pd,
    // set vec to zero
    _mm256_setzero_pd,
    // shift vector left and insert zeros
    _mm256_slli_si256,
    // Vec -> Memory but slower
    _mm256_storeu_pd,
};

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn scan_sse(mut x: __m256d) -> __m256d {
    // its "equivalent" to
    // x += x << (4 * 8);
    // x += x << (8 * 8);
    //
    // first pass:
    //      f4,      f3,      f2, f1 +
    //      f3,      f2,      f1,  0 =
    //     f43,     f32,     f21, f1
    //
    // second pass
    // f43, f32, f21, f1 +
    // f21,  f1,   0,  0 =
    // f4321, f321, f21, f1
    //
    // -> Fast cumulative sum using 2 adds and 2 shifts instead of (3 + 2 + 1) = 6 adds
    unsafe {
        x = _mm256_add_pd(
            x,
            _mm256_castsi256_pd(_mm256_slli_si256(_mm256_castpd_si256(x), 4)),
        );
        x = _mm256_add_pd(
            x,
            _mm256_castsi256_pd(_mm256_slli_si256(_mm256_castpd_si256(x), 8)),
        );
    }
    x
}

// TODO! debug
#[cfg(target_arch = "x86_64")]
pub fn cumsum_f64_avx_intrinsics(random_vec: &mut [f64]) {
    if random_vec.len() <= 1 {
        return;
    }
    // TODO WIP to fix
    unsafe {
        let mut offset: __m256d = _mm256_setzero_pd();
        for i in (0..random_vec.len()).step_by(4) {
            // it should be __mm_load_ps but if the values are not aligned it
            // raises a seg-fault so we use the slower _mm_loadu_ps until we figure
            // out how to ensure the alignmenet of the vector
            // loat the 4 values
            let x: __m256d = _mm256_loadu_pd(random_vec.as_ptr().wrapping_add(i));
            // compute the local cumulative sum
            let mut out: __m256d = scan_sse(x);
            // add the local cumulative sum to the current offset
            out = _mm256_add_pd(out, offset);
            // get the internal floats array of the result vec
            let ptr: *mut f64 = random_vec.as_mut_ptr();
            // store the value in the vector
            _mm256_storeu_pd(ptr.add(i), out);
            // Update the current offset (aka the last value of out)
            let t0: __m256d = _mm256_permute2f128_pd(out, out, 0x11);
            offset = _mm256_permute_pd(t0, 0x0f);
        }
    }

    // fix the last values if the array's length is not a multiple of 4
    let n = random_vec.len() - (random_vec.len() % 4);
    let offset = if n == 0 { 0.0 } else { random_vec[n - 1] };
    match random_vec.len() % 4 {
        1 => {
            random_vec[n] += offset;
        }
        2 => {
            random_vec[n] += offset;
            random_vec[n + 1] += random_vec[n];
        }
        3 => {
            random_vec[n] += offset;
            random_vec[n + 1] += random_vec[n];
            random_vec[n + 2] += random_vec[n + 1];
        }
        _ => {}
    };
}
