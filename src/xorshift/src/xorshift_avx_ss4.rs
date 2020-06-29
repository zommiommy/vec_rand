

// FASTEST
pub fn xorshift_avx_ss4(seed: & mut [u64; 16]) {
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rdi]\n",
            "vmovdqu ymm2, ymmword ptr [rdi + 32]\n",
            "vmovdqu ymm4, ymmword ptr [rdi + 64]\n",
            "vmovdqu ymm6, ymmword ptr [rdi + 96]\n",
            // tmp = seed << 13
            "vpsllq ymm1, ymm0, 13\n",
            "vpsllq ymm3, ymm2, 13\n",
            "vpsllq ymm5, ymm4, 13\n",
            "vpsllq ymm7, ymm6, 13\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // tmp = seed >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            "vpsrlq ymm3, ymm2, 7\n",
            "vpsrlq ymm5, ymm4, 7\n",
            "vpsrlq ymm7, ymm6, 7\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // tmp = seed << 17
            "vpsllq ymm1, ymm0, 17\n",
            "vpsllq ymm3, ymm2, 17\n",
            "vpsllq ymm5, ymm4, 17\n",
            "vpsllq ymm7, ymm6, 17\n",
            // seed ^= tmp
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0\n",
            "vmovdqu ymmword ptr [rdi + 32], ymm2\n",
            "vmovdqu ymmword ptr [rdi + 64], ymm4\n",
            "vmovdqu ymmword ptr [rdi + 96], ymm6\n"
        ),
        inout("rdi") seed => _,
        );
    }
}
