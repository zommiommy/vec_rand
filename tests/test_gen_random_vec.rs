extern crate vec_rand;

#[test]
fn test_gen_random_vec() {
    // test correct len
    for len in 1..100 {
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