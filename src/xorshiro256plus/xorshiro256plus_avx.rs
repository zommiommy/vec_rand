use core::arch::asm;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
/// Generate 4 random u64 by running 4 parallel instances of xorshift256plus using avx instructions.
///
/// Example:
///
/// ```ignore
/// use vec_rand::*;
/// let mut seed: [u64; 16] = [
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
/// ];
/// let values = xorshiro256plus_avx(& mut seed);
/// println!("{:?}", values);
/// ```
pub fn xorshiro256plus_avx(seed: &mut [u64; 16]) -> [u64; 4] {
    let mut result: [u64; 4] = [0; 4];
    unsafe {
        asm!(
        concat!(
            "vmovdqu ymm3, ymmword ptr [{0} + 96]\n",
            "vmovdqu ymm0, ymmword ptr [{0}]\n",
            "vmovdqu ymm1, ymmword ptr [{0} + 32]\n",
            "vpaddq ymm4, ymm0, ymm3\n",
            "vmovdqu ymmword ptr [{1}], ymm4\n",
            "vpsllq ymm4, ymm1, 17\n",
            "vmovdqu ymm2, ymmword ptr [{0} + 64]\n",
            "vpxor ymm2, ymm2, ymm0\n",
            "vpxor ymm3, ymm3, ymm1\n",
            "vpxor ymm1, ymm1, ymm2\n",
            "vmovdqu ymmword ptr [{0} + 32], ymm1\n",
            "vpxor ymm0, ymm0, ymm3\n",
            "vmovdqu ymmword ptr [{0}], ymm0\n",
            "vpxor ymm2, ymm2, ymm4\n",
            "vmovdqu ymmword ptr [{0} + 64], ymm2\n",
            // s[3] = (s[3] << 45) | (s[3] >> 19)
            // the or is only for avx512 so we use the xor
            // which is equivalent since both vpsllq and vpsrlq insert zeros
            "vpsllq ymm0, ymm1, 45\n",
            "vpsrlq ymm2, ymm1, 19\n",
            "vpxor ymm0, ymm0, ymm2\n",
            "vmovdqu ymmword ptr [rsi + 96], ymm0\n",
        ),
        inout(reg) seed => _,
        inout(reg) result.as_mut_ptr() => _,
        out("ymm0") _,
        out("ymm1") _,
        out("ymm2") _,
        out("ymm3") _,
        out("ymm4") _,
        );
    }
    result
}
