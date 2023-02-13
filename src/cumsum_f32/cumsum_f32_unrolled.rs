use core::iter::Iterator;

#[inline]
pub fn cumsum_f32_unrolled(random_vec: &mut [f32]) {
    if random_vec.len() <= 0{
        return;
    }

    let mut offset = 0.0f32;

    let max = if random_vec.len() >= 4 {
        if random_vec.len() % 4 == 0 {
            random_vec.len()
        } else {
            random_vec.len() - 4
        }
    } else {
        0
    };
    for i in (0..max).step_by(4){
        let mut a = random_vec[i];
        let mut b = random_vec[i+1];
        let mut c = random_vec[i+2];
        let mut d = random_vec[i+3];

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
