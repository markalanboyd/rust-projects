pub fn round_to(value: f64, places: i32) -> f64 {
    let multiplier = 10f64.powi(places);
    let rounded = (value * multiplier).round() / multiplier;
    return rounded;
}
