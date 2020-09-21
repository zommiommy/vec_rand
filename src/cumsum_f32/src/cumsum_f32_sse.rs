

#[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
pub fn cumsum_f32_sse(random_vec: &Vec<f32>) -> Vec<f32> {
    if random_vec.len() == 0{
        return vec![];
    }
    if random_vec.len() == 1{
        return random_vec.clone();
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
        asm!(
        concat!(
            // offset = 0
            "xorps xmm0, xmm0\n",
            "xor rcx, rcx\n",
            // for start
"for_start: cmp rbx, rcx\n",
            "jle for_end\n",
                // Load the data
                // let mut x: __m128 = _mm_loadu_ps(random_vec.as_ptr().wrapping_offset(i as isize));
                "movups xmm1, xmmword ptr [rsi]\n",
                // compute the partial sum
                // x = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 4)));
                //"pslldq xmm1, xmm2, 4\n",
                //"addps xmm1, xmm2, xmm1\n",
                // let mut out = _mm_add_ps(x, _mm_castsi128_ps(_mm_slli_si128(_mm_castps_si128(x), 8)));
                //"pslldq xmm1, xmm2, 8\n",
                //"addps xmm1, xmm2, xmm1\n",
                // add the result with the offset
                // out = _mm_add_ps(out, offset);
                //"addps xmm1, xmm0, xmm1\n",
                // Store the data
                // _mm_storeu_ps(result.as_mut_ptr().offset(i as isize), out);
                "movups xmmword ptr [rdi], xmm1\n",
                // fix the offset
                // offset = _mm_shuffle_ps(out, out, 511);
                //"shufps 0x1ff, xmm0, xmm0\n",
                // update the counters
                "add rsi, 16\n",
                "add rdi, 16\n",
                "add rcx, 4\n",
            // loop to the start of the for
            "jmp for_start\n",
"for_end:   movups xmmword ptr [r8], xmm0\n"
        ),
        inout("rbx") max >> 2 => _,
        inout("rsi") random_vec.as_ptr() => _,
        inout("rdi") result.as_mut_ptr() => _,
        inout("r8") _final_offset.as_mut_ptr() => _,
        );
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