pub fn dot(input: (i8, i8, i8), weights: (f64, f64, f64)) -> f64 {
    input.0 as f64 * weights.0 + input.1 as f64 * weights.1 + input.2 as f64 * weights.2
}
