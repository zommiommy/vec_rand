extern crate vec_rand;

const SIZE: usize = 10_069;
const ITER: usize = 1_000;


#[test]
fn test_sample() {
    for _ in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample(& mut weights);
        assert!(i >= 0);
        assert!(i < weights.len());
    }
}

#[test]
fn test_sample_plain() {
    for _ in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample_plain(& mut weights);
        assert!(i >= 0);
        assert!(i < weights.len());
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_sample_avx() {
    for _ in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample_avx(& mut weights);
        assert!(i >= 0);
        assert!(i < weights.len());
    }
}