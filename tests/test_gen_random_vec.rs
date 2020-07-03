extern crate vec_rand;

const START: usize = 1;
const END: usize = 10000;


#[test]
fn test_gen_random_vec() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let A = vec_rand::gen_random_vec(1000, 0xBAD5EED);
    let B = vec_rand::gen_random_vec(1000, 0xBAD5EED);

    for (a, b) in A.iter().zip(B.iter()) {
        assert_eq!(a, b);
    }
}


#[test]
fn test_gen_random_vec_4_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_4_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let A = vec_rand::gen_random_vec_4_1(1000, 0xBAD5EED);
    let B = vec_rand::gen_random_vec_4_1(1000, 0xBAD5EED);

    for (a, b) in A.iter().zip(B.iter()) {
        assert_eq!(a, b);
    }
}

#[test]
fn test_gen_random_vec_32_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_32_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let A = vec_rand::gen_random_vec_32_1(1000, 0xBAD5EED);
    let B = vec_rand::gen_random_vec_32_1(1000, 0xBAD5EED);

    for (a, b) in A.iter().zip(B.iter()) {
        assert_eq!(a, b);
    }
}

#[test]
fn test_gen_random_vec_32_4_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_32_4_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let A = vec_rand::gen_random_vec_32_4_1(1000, 0xBAD5EED);
    let B = vec_rand::gen_random_vec_32_4_1(1000, 0xBAD5EED);

    for (a, b) in A.iter().zip(B.iter()) {
        assert_eq!(a, b);
    }
}

#[test]
fn test_gen_random_vec_1() {
    // test correct len
    for len in START..END {
        let values = vec_rand::gen_random_vec_1(len, 0xBAD5EED);
        assert_eq!(values.len(), len);
    }


    // test reproducibility
    let A = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);
    let B = vec_rand::gen_random_vec_1(1000, 0xBAD5EED);

    for (a, b) in A.iter().zip(B.iter()) {
        assert_eq!(a, b);
    }
}