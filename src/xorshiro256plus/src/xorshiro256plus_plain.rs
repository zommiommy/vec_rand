
#[inline(always)]
fn rotl(x : u64, k: u64) -> u64{
	return (x << k) | (x >> (64 - k));
}

#[inline(always)]
pub fn xorshiro256plus_plain(s: & mut [u64; 4]) -> u64{
    let result = s[0] + s[3];
    let t: u64 = s[1] << 17;
    s[2] ^= s[0];
    s[3] ^= s[1];
    s[1] ^= s[2];
    s[0] ^= s[3];
    
    s[2] ^= t;

    s[3] = rotl(s[3], 45);

    result
}