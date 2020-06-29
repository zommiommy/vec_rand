
pub fn xorshiro256plus_avx(seed: & mut [u64; 20]) -> [u64; 4] {
    let mut result: [u64; 4] = [0; 4];
    unsafe {
        asm!(
        concat!(
            "vmovdqu ymm3, ymmword ptr [rsi + 96]\n",   
            "vmovdqu ymm0, ymmword ptr [rsi]\n",      
            "vmovdqu ymm1, ymmword ptr [rsi + 32]\n",  
            "vpaddq ymm4, ymm0, ymm3\n",              
            "vmovdqu ymmword ptr [rdi], ymm4\n",
            "vpsllq ymm4, ymm1, 17\n",                  
            "vmovdqu ymm2, ymmword ptr [rsi + 64]\n",
            "vpxor ymm2, ymm2, ymm0\n",   
            "vpxor ymm3, ymm3, ymm1\n",      
            "vpxor ymm1, ymm1, ymm2\n",                
            "vmovdqu ymmword ptr [rsi + 32], ymm1\n",  
            "vpxor ymm0, ymm0, ymm3\n",                
            "vmovdqu ymmword ptr [rsi], ymm0\n",     
            "vpxor ymm2, ymm2, ymm4\n",                 
            "vmovdqu ymmword ptr [rsi + 64], ymm2\n",  
            // s[3] = (s[3] << 45) | (s[3] >> 19)
            // the or is only for avx512 so we use the xor
            // which is equivalent since both vpsllq and vpsrlq insert zeros
            "vpsllq ymm0, ymm1, 45\n",
            "vpsrlq ymm2, ymm1, 19\n",
            "vpxor ymm0, ymm0, ymm2\n",
            "vmovdqu ymmword ptr [rsi + 96], ymm0\n",
        ),
        inout("rsi") seed => _,
        inout("rdi") result.as_mut_ptr() => _,
        );
    }
    result
}