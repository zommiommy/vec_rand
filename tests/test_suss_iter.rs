extern crate vec_rand;
use vec_rand::IteratorSuss;
use vec_rand::splitmix64;


#[test]
fn test_suss_iter() {
    for i in 0..100 {
        let seed = splitmix64(0xbad5eed_u64.wrapping_mul(i));
        let suss = (0..95)
            .suss(10, seed, None);
        let res = suss.collect::<Vec<_>>();
        println!("{:?}", res);
        assert_eq!(res.len(), 10);
    }
}
