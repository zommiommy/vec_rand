
#[cfg(feature="alloc")]
use ::alloc::vec::Vec;

#[cfg(feature="alloc")]
use ::alloc::*;


#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
use core::arch::x86_64::{
    // info can be found at https://software.intel.com/sites/landingpage/IntrinsicsGuide
    __m128,
    // sum two vector of f32
    _mm_add_ps,
    // cast __m128 to __m128i
    // see _mm_castsi128_ps
    _mm_castps_si128,
    // cast __m128i  to __m128
    // it's only for compilation, it does not gen instructions
    _mm_castsi128_ps,
    // Memory -> Vec but slower
    _mm_loadu_ps,
    // set vec to zero
    _mm_setzero_ps,
    // Shiffle the vecotr according to the mask given
    _mm_shuffle_ps,
    // shift vector left and insert zeros
    _mm_slli_si128,
    // Vec -> Memory but slower
    _mm_storeu_ps,
};

#[cfg(feature="alloc")]
#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
#[inline]
pub fn cumsum_f32_sse_intrinsics(random_vec: &[f32]) -> Vec<f32> {
    if random_vec.len() == 0{
        return Vec::new();
    }
    if random_vec.len() == 1{
        return random_vec.to_vec();
    }

    let mut result = vec![0.0f32; random_vec.len()];
    let mut _final_offset = vec![0.0f32; 4];

    let max = if random_vec.len() >= 4 {
        if random_vec.len() % 4 == 0 {
            random_vec.len()
        } else {
            random_vec.len() - 4
        }
    } else {
        0
    };
    unsafe {
        let mut offset: __m128 = _mm_setzero_ps();
        for i in (0..max).step_by(4) {
            // it should be __mm_load_ps but if the values are not aligned it
            // raises a seg-fault so we use the slower _mm_loadu_ps until we figure
            // out how to ensure the alignmenet of the vector
            // loat the 4 values
            let mut x: __m128 = _mm_loadu_ps(random_vec.as_ptr().wrapping_offset(i as isize));
            // compute the local cumulative sum
            x = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 4)));
            // TODO! ccheck tyhis line because slli_si128 must have values between 0 and 7 and we pass it 8
            let mut out = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 8)));
            // add the local cumulative sum to the current offset
            out = _mm_add_ps(out, offset);
            // get the internal floats array of the result vec
            let ptr: *mut f32 = result.as_mut_ptr();
            // store the value in the vector
            _mm_storeu_ps(ptr.offset(i as isize), out);
            // the mask should be  _MM_SHUFFLE(3, 3, 3, 3)
            // but it's unstable in rust so we manually embed it
            // as 511 because we want to broadcast the last value
            // to all 4 lanes, and the value is choosen with the first 8 bits
            // and since we want 3, 3, 3, 3, it's 8 bits set to 1 so
            // 2**9 - 1 = 511
            // Update the current offset (aka the last value of out)
            offset = _mm_shuffle_ps(out, out, 255); 
        }
        _mm_storeu_ps(_final_offset.as_mut_ptr(), offset);
    }

    let n = random_vec.len() -  (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            result[n] = random_vec[n] + _final_offset[0];
        },
        2 => {
            result[n] = random_vec[n] + _final_offset[0];
            result[n+1] = random_vec[n+1] + result[n];
        },
        3 => {
            result[n] = random_vec[n] + _final_offset[0];
            result[n+1] = random_vec[n+1] + result[n];
            result[n+2] = random_vec[n+2] + result[n + 1];
        },
        _ => {},
    };
    result
}