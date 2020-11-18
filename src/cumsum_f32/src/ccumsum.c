// https://github.com/joelangeway/CumulativeSum/blob/master/cumsum.c

#include <stdio.h>
#include <stdlib.h>
#include <sys/time.h>
#include <stdint.h>
#include <math.h>

#if _WIN32
	#include <immintrin.h>
#else
    #include <x86intrin.h>
#endif

/*
	The palignr instruction works just fine on vectors of floats, but the
	instrinsics available in my version of gcc don't seem to believe this.
	This macro just hides some ugly casting. It depends crusially on the fact
	that casting a __m128 to a __m128i and back works more like this:
		float x = 0.5;
		int y = *(int *)(&x);
		float z = *(float *)(&y);
	Than this:
		float x = 10;
		int y = (int)x;
		float z = (float)y;
	That is, it does not change the bits in the regesters at all, only the
	compiler's attitude towards those bits.
*/
#define _mm_alignr_ps(xmm1, xmm2, imm) ( _mm_castsi128_ps(_mm_alignr_epi8( _mm_castps_si128(xmm1),_mm_castps_si128(xmm2), (imm))))

extern void cumsum_super_scaler_simd(float *a, int n)
{
	/*
		This function computes an in-place cumulative sum.
		It is super scalar and SIMD. 
		It does 88 addition operations per 24 elements, but has only 6 addition
		operations in the longest chain of dependancies per 24 elements.
	*/
	__m128 a0, a1, a2, a3, a4, a5,
			s0, s1, s2, s3, s4, s5,
			b0,
			t, z;
	z = _mm_setzero_ps();
	t = z;

	while(((uint64_t)a & 15) && n > 0)
	{
		a0 = _mm_load_ss(a);
		t = _mm_add_ps(t, a0);
		_mm_store_ss(a, t);
		a++;
		n--;
	}
	while(n >= 24)
	{
		//load values
		a0 = _mm_load_ps(a);
		a1 = _mm_load_ps(a + 4);
		a2 = _mm_load_ps(a + 8);
		a3 = _mm_load_ps(a + 12);
		a4 = _mm_load_ps(a + 16);
		a5 = _mm_load_ps(a + 20);

		//start by adding in running total
		a0 = _mm_add_ps(a0, t);

		//four separate streams of cumsum each done in parallel with the super scaler method
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
		s0 = _mm_alignr_ps(a0, z, 12);
		s1 = _mm_alignr_ps(a1, a0, 12);
		s2 = _mm_alignr_ps(a2, a1, 12);
		s3 = _mm_alignr_ps(a3, a2, 12);
		s4 = _mm_alignr_ps(a4, a3, 12);
		s5 = _mm_alignr_ps(a5, a4, 12);
		
		a0 = _mm_add_ps(a0, s0);
		a1 = _mm_add_ps(a1, s1);
		a2 = _mm_add_ps(a2, s2);
		a3 = _mm_add_ps(a3, s3);
		a4 = _mm_add_ps(a4, s4);
		a5 = _mm_add_ps(a5, s5);

		//shift right by two elements and add
		s0 = _mm_alignr_ps(a0, z, 8);
		s1 = _mm_alignr_ps(a1, a0, 8);
		s2 = _mm_alignr_ps(a2, a1, 8);
		s3 = _mm_alignr_ps(a3, a2, 8);
		s4 = _mm_alignr_ps(a4, a3, 8);
		s5 = _mm_alignr_ps(a5, a4, 8);
		
		a0 = _mm_add_ps(a0, s0);
		a1 = _mm_add_ps(a1, s1);
		a2 = _mm_add_ps(a2, s2);
		a3 = _mm_add_ps(a3, s3);
		a4 = _mm_add_ps(a4, s4);
		a5 = _mm_add_ps(a5, s5);
		
		//reextract running total
		t = _mm_insert_ps(t, a5, _MM_SHUFFLE(3,0,0,14));

		_mm_store_ps(a, a0);
		_mm_store_ps(a + 4, a1);
		_mm_store_ps(a + 8, a2);
		_mm_store_ps(a + 12, a3);
		_mm_store_ps(a + 16, a4);
		_mm_store_ps(a + 20, a5);
		a += 24;
		n -= 24;
	}
	while(n > 0)
	{
		a0 = _mm_load_ss(a);
		t = _mm_add_ss(t, a0);
		_mm_store_ss(a, t);
		a++;
		n--;
	}
}

inline __m256 scan_AVX(__m256 x) {
    __m256 t0, t1;
    //shift1_AVX + add
    t0 = _mm256_permute_ps(x, _MM_SHUFFLE(2, 1, 0, 3));
    t1 = _mm256_permute2f128_ps(t0, t0, 41);
    x = _mm256_add_ps(x, _mm256_blend_ps(t0, t1, 0x11));
    //shift2_AVX + add
    t0 = _mm256_permute_ps(x, _MM_SHUFFLE(1, 0, 3, 2));
    t1 = _mm256_permute2f128_ps(t0, t0, 41);
    x = _mm256_add_ps(x, _mm256_blend_ps(t0, t1, 0x33));
    //shift3_AVX + add
    x = _mm256_add_ps(x,_mm256_permute2f128_ps(x, x, 41));
    return x;
}

extern void prefix_sum_AVX256(float *a, const int n) {
	__m256 t0;
    __m256 offset = _mm256_setzero_ps();
    for (int i = 0; i < n; i += 32) {
		// load the data
        __m256 x1 = _mm256_loadu_ps(&a[i]);
        __m256 x2 = _mm256_loadu_ps(&a[i + 8]);
        __m256 x3 = _mm256_loadu_ps(&a[i + 16]);
        __m256 x4 = _mm256_loadu_ps(&a[i + 24]);
		// perform the cumulative sum inside the single vectors
        __m256 out1 = scan_AVX(x1);
        __m256 out2 = scan_AVX(x2);
        __m256 out3 = scan_AVX(x3);
        __m256 out4 = scan_AVX(x4);
		// Add the offset and sum
        out1 = _mm256_add_ps(out1, offset);
        // save the alst value of the last vector as offset
        t0 = _mm256_permute2f128_ps(out1, out1, 0x11);
        offset = _mm256_permute_ps(t0, 0xff);

        out2 = _mm256_add_ps(out2, offset);

        t0 = _mm256_permute2f128_ps(out2, out2, 0x11);
        offset = _mm256_permute_ps(t0, 0xff);

        out3 = _mm256_add_ps(out3, offset);

        t0 = _mm256_permute2f128_ps(out3, out3, 0x11);
        offset = _mm256_permute_ps(t0, 0xff);

        out4 = _mm256_add_ps(out4, offset);

		// save the results
        _mm256_storeu_ps(&a[i], out1);
        _mm256_storeu_ps(&a[i + 8], out2);
        _mm256_storeu_ps(&a[i + 16], out3);
        _mm256_storeu_ps(&a[i + 24], out4);

        // save the alst value of the last vector as offset
        __m256 t0 = _mm256_permute2f128_ps(out4, out4, 0x11);
		// broadcast the value to all the values in the vector
        offset = _mm256_permute_ps(t0, 0xff);
    }   
}
