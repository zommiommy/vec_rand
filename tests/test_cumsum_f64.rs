extern crate vec_rand;

const SIZE: usize = 1031;
const ITER: usize = 1000;


#[test]
fn test_cumsum_f64_scan() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f64::cumsum_f64_plain(&weights);
        let tested = vec_rand::cumsum_f64::cumsum_f64_scan(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.0001);
        }
    }
}

#[test]
fn test_cumsum_f64_unrolled() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f64::cumsum_f64_plain(&weights);
        let tested = vec_rand::cumsum_f64::cumsum_f64_unrolled(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.0001);
        }
    }
}

#[test]
fn test_cumsum_f64_sse_intrinsics() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f64::cumsum_f64_plain(&weights);
        let tested = vec_rand::cumsum_f64::cumsum_f64_sse_intrinsics(&weights);

        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.0001);
        }
    }
}

#[test]
fn test_cumsum_f64() {
    for _ in 0..ITER {
        let weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);

        let truth = vec_rand::cumsum_f64::cumsum_f64_plain(&weights);
        let tested = vec_rand::cumsum_f64::cumsum_f64(&weights);
        for (a, b) in truth.iter().zip(tested.iter()) {
            assert!((a - b).abs() < 0.001);
        }
    }
}