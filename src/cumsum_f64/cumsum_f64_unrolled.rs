use core::iter::Iterator;

#[inline]
pub fn cumsum_f64_unrolled(random_vec: &mut [f64]){
    if random_vec.len() <= 1{
        return;
    }

    let mut offset = 0.0f64;

    let max = if random_vec.len() >= 4 {
        if random_vec.len() % 4 == 0 {
            random_vec.len()
        } else {
            random_vec.len() - 4
        }
    } else {
        0
    };

    // do the sum by chunks of 4 so that the complier can
    // vectorize the operation if avx or sse are available
    for i in (0..max).step_by(4){
        let mut a = random_vec[i];
        let mut b = random_vec[i+1];
        let mut c = random_vec[i+2];
        let mut d = random_vec[i+3];

        // doing the same sums multiple times to reduce
        // the data dependancy.
        d += c + b + a + offset;
        c += b + a + offset;
        b += a + offset;
        a += offset;

        random_vec[i] = a;
        random_vec[i+1] = b;
        random_vec[i+2] = c;
        random_vec[i+3] = d;

        offset = d;
    }

    // fix the last values if the array's length is not a multiple of 4
    let n = random_vec.len() -  (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            random_vec[n] = random_vec[n] + offset;
        },
        2 => {
            random_vec[n] = random_vec[n] + offset;
            random_vec[n+1] = random_vec[n+1] + random_vec[n];
        },
        3 => {
            random_vec[n] = random_vec[n] + offset;
            random_vec[n+1] = random_vec[n+1] + random_vec[n];
            random_vec[n+2] = random_vec[n+2] + random_vec[n + 1];
        },
        _ => {},
    };
}
