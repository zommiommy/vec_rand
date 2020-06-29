pub fn cumsum_f32_scan(random_vec: &Vec<f32>) -> Vec<f32> {
    random_vec
    .iter()
    .scan(0f32, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect()
}