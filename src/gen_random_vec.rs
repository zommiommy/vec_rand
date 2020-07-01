

pub fn gen_random_vec(size: usize) -> Vec<u64>{
    for i in (0..size).step_by(4){
        xorshift_avx_ss8(& mut seed)
    }
    let n = random_vec.len() -  (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            result[n] += result[n-1];
        },
        2 => {
            result[n] += result[n-1];
            result[n+1] += result[n];
        },
        3 => {
            result[n] += result[n-1];
            result[n+1] += result[n];
            result[n+2] += result[n+1];
        },
        _ => {},
    };
}