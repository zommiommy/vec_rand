use core::arch::x86_64::*;

macro_rules! _mm_alignr_ps {
    ($xmm1:expr, $xmm2:expr, $imm:literal) => {
        _mm_castsi128_ps(_mm_alignr_epi8::<$imm>(
            _mm_castps_si128($xmm1),
            _mm_castps_si128($xmm2),
        ))
    };
}

#[inline]
pub fn cumsum_super_scaler_simd(random_vec: &mut [f32]) {
    unsafe {
        const SHUFFLE_MASK: i32 = 206; //_MM_SHUFFLE(3,0,0,14);
        let zero = _mm_setzero_ps();
        let mut total = _mm_setzero_ps();

        let mut b0: __m128;

        let mut n = random_vec.len();
        let mut ptr = random_vec.as_mut_ptr();

        while n >= 24 {
            let mut a0 = _mm_load_ps(ptr);
            let mut a1 = _mm_load_ps(ptr.add(4));
            let mut a2 = _mm_load_ps(ptr.add(8));
            let mut a3 = _mm_load_ps(ptr.add(12));
            let mut a4 = _mm_load_ps(ptr.add(16));
            let mut a5 = _mm_load_ps(ptr.add(20));

            //start by adding in running total
            a0 = _mm_add_ps(a0, total);

            //four separate streams of cumsum each done in parallel with the
            // super scaler method
            a1 = _mm_add_ps(a1, a0);
            a3 = _mm_add_ps(a3, a2);
            a5 = _mm_add_ps(a5, a4);
            b0 = a3;
            a4 = _mm_add_ps(a4, b0);
            a5 = _mm_add_ps(a5, b0);
            a2 = _mm_add_ps(a2, a1);
            a3 = _mm_add_ps(a3, a1);
            a4 = _mm_add_ps(a4, a1);
            a5 = _mm_add_ps(a5, a1);
            //now join streams by adding previous three elements to each element

            //shift right by one element and add

            let mut s0 = _mm_alignr_ps!(a0, zero, 12);
            let mut s1 = _mm_alignr_ps!(a1, a0, 12);
            let mut s2 = _mm_alignr_ps!(a2, a1, 12);
            let mut s3 = _mm_alignr_ps!(a3, a2, 12);
            let mut s4 = _mm_alignr_ps!(a4, a3, 12);
            let mut s5 = _mm_alignr_ps!(a5, a4, 12);

            a0 = _mm_add_ps(a0, s0);
            a1 = _mm_add_ps(a1, s1);
            a2 = _mm_add_ps(a2, s2);
            a3 = _mm_add_ps(a3, s3);
            a4 = _mm_add_ps(a4, s4);
            a5 = _mm_add_ps(a5, s5);

            //shift right by two elements and add
            s0 = _mm_alignr_ps!(a0, zero, 8);
            s1 = _mm_alignr_ps!(a1, a0, 8);
            s2 = _mm_alignr_ps!(a2, a1, 8);
            s3 = _mm_alignr_ps!(a3, a2, 8);
            s4 = _mm_alignr_ps!(a4, a3, 8);
            s5 = _mm_alignr_ps!(a5, a4, 8);

            a0 = _mm_add_ps(a0, s0);
            a1 = _mm_add_ps(a1, s1);
            a2 = _mm_add_ps(a2, s2);
            a3 = _mm_add_ps(a3, s3);
            a4 = _mm_add_ps(a4, s4);
            a5 = _mm_add_ps(a5, s5);

            //reextract running total
            total = _mm_insert_ps::<SHUFFLE_MASK>(total, a5);

            _mm_store_ps(ptr, a0);
            _mm_store_ps(ptr.add(4), a1);
            _mm_store_ps(ptr.add(8), a2);
            _mm_store_ps(ptr.add(12), a3);
            _mm_store_ps(ptr.add(16), a4);
            _mm_store_ps(ptr.add(20), a5);

            ptr = ptr.add(24);
            n -= 24;
        }

        while n > 0 {
            let a0 = _mm_load_ss(ptr);
            total = _mm_add_ss(total, a0);
            _mm_store_ss(ptr, total);
            ptr = ptr.add(1);
            n -= 1;
        }
    }
}
