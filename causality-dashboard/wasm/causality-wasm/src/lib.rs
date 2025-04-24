mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }

    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x.iter().map(|a| a * a).sum();
    let sum_y2: f64 = y.iter().map(|a| a * a).sum();

    let numerator = sum_xy - (sum_x * sum_y) / n;
    let denominator = ((sum_x2 - (sum_x * sum_x) / n) * (sum_y2 - (sum_y * sum_y) / n)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

#[wasm_bindgen]
pub fn process_text(text: &str) -> f64 {
    // Simple text processing example - returns length as f64
    text.len() as f64
}
