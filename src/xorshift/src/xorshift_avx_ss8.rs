
#[cfg(target_arch = "x86_64")]
#[inline(always)]
/// Generate 32 random u64 by running 32 parallel xorshifts using avx.
/// This method exploits both the SIMD instructions and the Out of Order Execution.
/// 
/// This is the method has a lower throughtput than the `xorshift_avx_ss4` because the
/// Decoder which can only fetch 16 bytes per clock cycle and most of avx instruction
/// are 4 bytes wide. 
/// 
/// Example:
/// 
/// ```
///  let mut seed: [u64; 32] = [
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///  ];
/// let values = xorshift_avx_ss8(& mut seed);
/// println!("{:?}", values);
/// ```
pub fn xorshift_avx_ss8(seed: & mut [u64; 32]) -> [u64; 32] {
    let mut result: [u64; 32] = [0; 32];
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0,  ymmword ptr [{0}]\n",
            "vmovdqu ymm2,  ymmword ptr [{0} + 32]\n",
            "vmovdqu ymm4,  ymmword ptr [{0} + 64]\n",
            "vmovdqu ymm6,  ymmword ptr [{0} + 96]\n",
            "vmovdqu ymm8,  ymmword ptr [{0} + 128]\n",
            "vmovdqu ymm10, ymmword ptr [{0} + 160]\n",
            "vmovdqu ymm12, ymmword ptr [{0} + 192]\n",
            "vmovdqu ymm14, ymmword ptr [{0} + 224]\n",
            // << 13
            "vpsllq ymm1, ymm0, 13\n",
            "vpsllq ymm3, ymm2, 13\n",
            "vpsllq ymm5, ymm4, 13\n",
            "vpsllq ymm7, ymm6, 13\n",
            "vpsllq ymm9, ymm8, 13\n",
            "vpsllq ymm11, ymm10, 13\n",
            "vpsllq ymm13, ymm12, 13\n",
            "vpsllq ymm15, ymm14, 13\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            "vpsrlq ymm3, ymm2, 7\n",
            "vpsrlq ymm5, ymm4, 7\n",
            "vpsrlq ymm7, ymm6, 7\n",
            "vpsrlq ymm9, ymm8, 7\n",
            "vpsrlq ymm11, ymm10, 7\n",
            "vpsrlq ymm13, ymm12, 7\n",
            "vpsrlq ymm15, ymm14, 7\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // << 17
            "vpsllq ymm1, ymm0, 17\n",
            "vpsllq ymm3, ymm2, 17\n",
            "vpsllq ymm5, ymm4, 17\n",
            "vpsllq ymm7, ymm6, 17\n",
            "vpsllq ymm9, ymm8, 17\n",
            "vpsllq ymm11, ymm10, 17\n",
            "vpsllq ymm13, ymm12, 17\n",
            "vpsllq ymm15, ymm14, 17\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            "vpxor ymm2, ymm2, ymm3\n",
            "vpxor ymm4, ymm4, ymm5\n",
            "vpxor ymm6, ymm6, ymm7\n",
            "vpxor ymm8, ymm9, ymm1\n",
            "vpxor ymm10, ymm11, ymm3\n",
            "vpxor ymm12, ymm13, ymm5\n",
            "vpxor ymm14, ymm15, ymm7\n",
            // Store the data
            "vmovdqu ymmword ptr [{1}], ymm0\n",
            "vmovdqu ymmword ptr [{1} + 32], ymm2\n",
            "vmovdqu ymmword ptr [{1} + 64], ymm4\n",
            "vmovdqu ymmword ptr [{1} + 96], ymm6\n",
            "vmovdqu ymmword ptr [{1} + 128], ymm8\n",
            "vmovdqu ymmword ptr [{1} + 160], ymm10\n",
            "vmovdqu ymmword ptr [{1} + 192], ymm12\n",
            "vmovdqu ymmword ptr [{1} + 224], ymm14\n",
            // modify the seed
            "vmovdqu ymmword ptr [{0}], ymm0\n",
            "vmovdqu ymmword ptr [{0} + 32], ymm2\n",
            "vmovdqu ymmword ptr [{0} + 64], ymm4\n",
            "vmovdqu ymmword ptr [{0} + 96], ymm6\n",
            "vmovdqu ymmword ptr [{0} + 128], ymm8\n",
            "vmovdqu ymmword ptr [{0} + 160], ymm10\n",
            "vmovdqu ymmword ptr [{0} + 192], ymm12\n",
            "vmovdqu ymmword ptr [{0} + 224], ymm14\n"
        ),
        inout(reg) seed => _,
        inout(reg) result.as_mut_ptr() => _,
        out("ymm0") _,
        out("ymm1") _,
        out("ymm2") _,
        out("ymm3") _,
        out("ymm4") _,
        out("ymm5") _,
        out("ymm6") _,
        out("ymm7") _,
        out("ymm8") _,
        out("ymm9") _,
        out("ymm10") _,
        out("ymm11") _,
        out("ymm12") _,
        out("ymm13") _,
        out("ymm14") _,
        );
    }
    result
}