

#[inline(always)]
/// Generate 4 random u64 by running 4 parallel xorshifts using avx.
/// 
/// Example:
/// 
/// ```
///  let mut seed: [u64; 4] = [
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///      0xBAD5EEDdeadbeef,
///  ];
/// let values = xorshift_avx(& mut seed);
/// println!("{:?}", values);
/// ```
pub fn xorshift_avx(seed: & mut [u64; 4]) -> [u64; 4] {
    let mut result: [u64; 4] = [0; 4];
    unsafe {
        asm!(
        concat!(
            // Load the data
            "vmovdqu ymm0, ymmword ptr [rsi]\n",
            // << 13
            "vpsllq ymm1, ymm0, 13\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // >> 7
            "vpsrlq ymm1, ymm0, 7\n",
            // ^
            "vpxor ymm0, ymm0, ymm1\n",
            // << 17
            "vpsllq ymm1, ymm0, 17\n",
            // ^c
            "vpxor ymm0, ymm0, ymm1\n",
            // Store the data
            "vmovdqu ymmword ptr [rdi], ymm0\n",
            "vmovdqu ymmword ptr [rsi], ymm0\n"
        ),
        inout("rsi") seed => _,
        inout("rdi") result.as_mut_ptr() => _,
        );
    }
    result
}