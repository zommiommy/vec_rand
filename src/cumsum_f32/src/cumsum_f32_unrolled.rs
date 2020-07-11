
pub fn cumsum_f32_unrolled(random_vec: &Vec<f32>) -> Vec<f32> {
    if random_vec.len() == 0{
        return vec![];
    }
    if random_vec.len() == 1{
        return random_vec.clone();
    }

    let mut result = vec![0.0f32; random_vec.len()];
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

        result[i] = a;
        result[i+1] = b;
        result[i+2] = c;
        result[i+3] = d;

        offset = d;
    }

    let n = random_vec.len() -  (random_vec.len() % 4);
    match random_vec.len() % 4 {
        1 => {
            result[n] = random_vec[n] + offset;
        },
        2 => {
            result[n] = random_vec[n] + offset;
            result[n+1] = random_vec[n+1] + result[n];
        },
        3 => {
            result[n] = random_vec[n] + offset;
            result[n+1] = random_vec[n+1] + result[n];
            result[n+2] = random_vec[n+2] + result[n + 1];
        },
        _ => {},
    };
    
    result
}
