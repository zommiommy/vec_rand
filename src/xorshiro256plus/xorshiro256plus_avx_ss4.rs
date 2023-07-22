use core::arch::asm;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
/// Generate 16 random u64 by running 16 parallel instances of xorshift256plus using avx instructions and exploiting OOO Execution (Out Of Order).
///
/// Example:
///
/// ```
/// use vec_rand::*;
/// let mut seed: [u64; 64] = [
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
///     6591408588322595484, 5451729388608518856, 8913376598984957243, 17912695770704705270,
/// ];
/// let values = xorshiro256plus_avx_ss4(& mut seed);
/// println!("{:?}", values);
/// ```
pub fn xorshiro256plus_avx_ss4(seed: &mut [u64; 64]) -> [u64; 16] {
    let mut result: [u64; 16] = [0; 16];
    // for info about the scheduling of registers / operation
    //I made https://docs.google.com/spreadsheets/d/1tOgA91OVw9GBKVIXVDeAsQKar3IMXAZXBn3aZwdeDug/edit?usp=sharing
    unsafe {
        asm!(
        concat!(
            // a = s[0]
            "vmovdqu ymm0, ymmword ptr [{0}]\n",
            "vmovdqu ymm1, ymmword ptr [{0} + 32]\n",
            "vmovdqu ymm2, ymmword ptr [{0} + 64]\n",
            "vmovdqu ymm3, ymmword ptr [{0} + 96]\n",
            // b = s[3]
            "vmovdqu ymm4, ymmword ptr [{0} + 128]\n",
            "vmovdqu ymm5, ymmword ptr [{0} + 160]\n",
            "vmovdqu ymm6, ymmword ptr [{0} + 192]\n",
            "vmovdqu ymm7, ymmword ptr [{0} + 224]\n",
            // c = a + b
            "vpaddq ymm8,  ymm0, ymm4\n",
            "vpaddq ymm9,  ymm1, ymm5\n",
            "vpaddq ymm10, ymm2, ymm6\n",
            "vpaddq ymm11, ymm3, ymm7\n",
            // result = c
            "vmovdqu ymmword ptr [{1}], ymm8\n",
            "vmovdqu ymmword ptr [{1} + 32], ymm9\n",
            "vmovdqu ymmword ptr [{1} + 64], ymm10\n",
            "vmovdqu ymmword ptr [{1} + 96], ymm11\n",
            // d = s[2]
            "vmovdqu ymm12, ymmword ptr [{0} + 256]\n",
            "vmovdqu ymm13, ymmword ptr [{0} + 288]\n",
            "vmovdqu ymm14, ymmword ptr [{0} + 320]\n",
            "vmovdqu ymm15, ymmword ptr [{0} + 352]\n",
            // c = a ^ d
            "vpxor ymm8,  ymm0, ymm4\n",
            "vpxor ymm9,  ymm1, ymm5\n",
            "vpxor ymm10, ymm2, ymm6\n",
            "vpxor ymm11, ymm3, ymm7\n",
            // d = s[1]
            "vmovdqu ymm12, ymmword ptr [{0} + 128]\n",
            "vmovdqu ymm13, ymmword ptr [{0} + 160]\n",
            "vmovdqu ymm14, ymmword ptr [{0} + 192]\n",
            "vmovdqu ymm15, ymmword ptr [{0} + 224]\n",
            // b = b ^ d
            "vpxor ymm4, ymm4, ymm12\n",
            "vpxor ymm5, ymm5, ymm13\n",
            "vpxor ymm6, ymm6, ymm14\n",
            "vpxor ymm7, ymm7, ymm15\n",
            // a = a ^ b
            "vpxor ymm0, ymm0, ymm4\n",
            "vpxor ymm1, ymm1, ymm5\n",
            "vpxor ymm2, ymm2, ymm6\n",
            "vpxor ymm3, ymm3, ymm7\n",
            // r[0] = a
            "vmovdqu ymmword ptr [{0} + 0],  ymm0\n",
            "vmovdqu ymmword ptr [{0} + 32], ymm1\n",
            "vmovdqu ymmword ptr [{0} + 64], ymm2\n",
            "vmovdqu ymmword ptr [{0} + 96], ymm3\n",
            // a = c ^ d
            "vpxor ymm0, ymm8,  ymm12\n",
            "vpxor ymm1, ymm9,  ymm13\n",
            "vpxor ymm2, ymm10, ymm14\n",
            "vpxor ymm3, ymm11, ymm15\n",
            // r[1] = a
            "vmovdqu ymmword ptr [{0} + 128], ymm0\n",
            "vmovdqu ymmword ptr [{0} + 160], ymm1\n",
            "vmovdqu ymmword ptr [{0} + 192], ymm2\n",
            "vmovdqu ymmword ptr [{0} + 224], ymm3\n",
            // d = d << 17
            "vpsllq ymm12, ymm12, 17\n",
            "vpsllq ymm13, ymm13, 17\n",
            "vpsllq ymm14, ymm14, 17\n",
            "vpsllq ymm15, ymm15, 17\n",
            // a = c ^ d
            "vpxor ymm0, ymm8,  ymm12\n",
            "vpxor ymm1, ymm9,  ymm13\n",
            "vpxor ymm2, ymm10, ymm14\n",
            "vpxor ymm3, ymm11, ymm15\n",
            // r[2] = a
            "vmovdqu ymmword ptr [{0} + 256], ymm0\n",
            "vmovdqu ymmword ptr [{0} + 288], ymm1\n",
            "vmovdqu ymmword ptr [{0} + 320], ymm2\n",
            "vmovdqu ymmword ptr [{0} + 352], ymm3\n",
            // a = b << 45
            "vpsllq ymm0, ymm4, 45\n",
            "vpsllq ymm1, ymm5, 45\n",
            "vpsllq ymm2, ymm6, 45\n",
            "vpsllq ymm3, ymm7, 45\n",
            // c = b >> 19
            "vpsrlq ymm8,  ymm4, 19\n",
            "vpsrlq ymm9,  ymm5, 19\n",
            "vpsrlq ymm10, ymm6, 19\n",
            "vpsrlq ymm11, ymm7, 19\n",
            // d = a ^ c
            "vpxor ymm12, ymm0, ymm8\n",
            "vpxor ymm13, ymm1, ymm9\n",
            "vpxor ymm14, ymm2, ymm10\n",
            "vpxor ymm15, ymm3, ymm11\n",
            // r[3] = d
            "vmovdqu ymmword ptr [{0} + 384], ymm12\n",
            "vmovdqu ymmword ptr [{0} + 416], ymm13\n",
            "vmovdqu ymmword ptr [{0} + 448], ymm14\n",
            "vmovdqu ymmword ptr [{0} + 480], ymm15\n"
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
        out("ymm15") _,
        );
    }
    result
}
