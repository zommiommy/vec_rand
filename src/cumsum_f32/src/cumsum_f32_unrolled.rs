
pub fn cumsum_f32_unrolled(random_vec: &Vec<f32>) -> Vec<f32> {
    let mut result = vec![0.0f32; 4 * ( random_vec.len() as f32 / 4.0).ceil() as usize];
    let mut offset = 0.0f32;

    for i in (0..random_vec.len()).step_by(4){
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

    for _ in 0..(random_vec.len() % 4){
        result.pop();
    }

    result
}
