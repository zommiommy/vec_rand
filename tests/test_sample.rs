extern crate vec_rand;

const SIZE: usize = 10_069;
const ITER: usize = 1_000;


#[test]
fn test_sample() {
    let mut values = vec![0; SIZE];
    for j in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample(& mut weights,0xBAD5eed + j as u64);
        assert!(i < weights.len());
        values[i] += 1;
    }
    let M = values.iter().max().unwrap();
    let m = values.iter().min().unwrap();
    println!("max {}, min{}", M, m);
    assert!((M - m) < 10);
}

#[test]
fn test_sample_plain() {
    let mut values = vec![0; SIZE];
    for j in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample_plain(& mut weights,0xBAD5eed + j as u64);
        assert!(i < weights.len());
        values[i] += 1;
    }
    let M = values.iter().max().unwrap();
    let m = values.iter().min().unwrap();
    println!("max {}, min{}", M, m);
    assert!((M - m) < 10);
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_sample_avx() {
    let mut values = vec![0; SIZE];
    for j in 0..ITER {
        let mut weights = vec_rand::gen_random_vec_f64(SIZE, 0xBAD5eed);
        let i = vec_rand::sample_avx(& mut weights, 0xBAD5eed + j as u64);
        assert!(i < weights.len());
        values[i] += 1;
    }
    let M = values.iter().max().unwrap();
    let m = values.iter().min().unwrap();
    println!("max {}, min{}", M, m);
    assert!((M - m) < 10);
}