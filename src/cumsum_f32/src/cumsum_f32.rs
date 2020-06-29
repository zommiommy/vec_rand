pub fn cumsum_f32(random_vec: &Vec<f32>) -> Vec<f32> {
    let mut cumulative_sum: Vec<f32> = Vec::with_capacity(random_vec.len());
    let mut total_weight = 0f32;
    for w in random_vec {
        total_weight += w;
        cumulative_sum.push(total_weight.clone());
    }
    cumulative_sum
}