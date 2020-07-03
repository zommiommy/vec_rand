extern crate vec_rand;

const SIZE: usize = 10_000;

#[test]
fn test_u64_to_f64_mul() {
    for x in &vec_rand::gen_random_vec(SIZE, 0xBAD5eed) {
        let converted = vec_rand::u64_to_f64_mul(*x);
        assert!(converted >= 0.0);
        assert!(converted <= 1.0);
    }
}

#[test]
fn test_u64_to_f64_no_mul() {
    for x in &vec_rand::gen_random_vec(SIZE, 0xBAD5eed) {
        let converted = vec_rand::u64_to_f64_no_mul(*x);
        assert!(converted >= 0.0);
        assert!(converted <= 1.0);
    }
}