
pub fn cumsum_f64_unrolled_unsafe(random_vec: &mut Vec<f64>){
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
    for i in (0..max).step_by(4){
        unsafe{
            let mut a = *random_vec.get_unchecked_mut(i);
            let mut b = *random_vec.get_unchecked_mut(i + 1);
            let mut c = *random_vec.get_unchecked_mut(i + 2);
            let mut d = *random_vec.get_unchecked_mut(i + 3);

            d += c + b + a + offset;
            c += b + a + offset;
            b += a + offset;
            a += offset;

            *random_vec.get_unchecked_mut(i) = a;
            *random_vec.get_unchecked_mut(i + 1) = b;
            *random_vec.get_unchecked_mut(i + 2) = c;
            *random_vec.get_unchecked_mut(i + 3) = d;

            offset = d;
        }
    }

    let n = random_vec.len() -  (random_vec.len() % 4);
    unsafe{
        match random_vec.len() % 4 {
            1 => {
                *random_vec.get_unchecked_mut(n) = *random_vec.get_unchecked_mut(n) + offset;
            },
            2 => {
                *random_vec.get_unchecked_mut(n) = *random_vec.get_unchecked_mut(n) + offset;
                *random_vec.get_unchecked_mut(n + 1) = *random_vec.get_unchecked_mut(n + 1) + *random_vec.get_unchecked_mut(n);
            },
            3 => {
                *random_vec.get_unchecked_mut(n) = *random_vec.get_unchecked_mut(n) + offset;
                *random_vec.get_unchecked_mut(n + 1) = *random_vec.get_unchecked_mut(n + 1) + *random_vec.get_unchecked_mut(n);
                *random_vec.get_unchecked_mut(n + 2) = *random_vec.get_unchecked_mut(n + 2) + *random_vec.get_unchecked_mut(n + 1);
            },
            _ => {},
        };
    }
}
