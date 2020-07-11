extern crate vec_rand;

const START: usize = 1;
const END: usize = 10069;


#[test]
fn test_gen_random_vec_f64() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_f64(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}

#[test]
fn test_gen_random_vec() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}


#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_gen_random_vec_4_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_4_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_gen_random_vec_32_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_32_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_gen_random_vec_32_4_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_32_4_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}

#[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
#[test]
fn test_gen_random_vec_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let step_a = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let step_b = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in step_a.iter().zip(step_b.iter()) {
        assert_eq!(a, b);
    }
}