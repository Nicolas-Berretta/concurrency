pub fn liebniz_series(terms: usize) -> f64 {
    let mut sum = 0.0;

    for i in 0..terms {
        let term = (-1.0_f64).powi(i as i32) / (2 * i + 1) as f64;
        sum += term;
    }
    4.0 * sum
}