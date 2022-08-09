use core::arch::asm;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
/// Generate 16 random u64 by running 16 parallel xorshifts using avx.
/// This method exploits both the SIMD instructions and the Out of Order Execution.
/// 
/// This is the method with the best throughtput because with saturate the Instruction
/// Decoder which can only fetch 16 bytes per clock cycle and most of avx instruction
/// are 4 bytes wide.
/// 
/// Example:
/// 
/// ```
///  let mut seed: [u64; 16] = [
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef, 0xBAD5EEDdeadbeef,
///  ];
/// let values = xorshift_avx_ss4(& mut seed);
/// println!("{:?}", values);
/// ```
pub fn xorshift_avx_ss4(seed: &[u64; 16]) -> [u64; 16]{
    let mut result: [u64; 16] = [0; 16];
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [{0}]\n",
            "vmovdqu ymm2, ymmword ptr [{0} + 32]\n",
            "vmovdqu ymm4, ymmword ptr [{0} + 64]\n",
            "vmovdqu ymm6, ymmword ptr [{0} + 96]\n",
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
            "vmovdqu ymmword ptr [{1}], ymm0\n",
            "vmovdqu ymmword ptr [{1} + 32], ymm2\n",
            "vmovdqu ymmword ptr [{1} + 64], ymm4\n",
            "vmovdqu ymmword ptr [{1} + 96], ymm6\n"
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
        );
    }
    result
}
