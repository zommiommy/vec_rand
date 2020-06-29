
pub fn cumsum_f64_scan(random_vec: &Vec<f64>) -> Vec<f64> {
    random_vec
    .iter()
    .scan(0f64, |acc, &x| {
        *acc = *acc + x;
        Some(*acc)
    })
    .collect()
}
