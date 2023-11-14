use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn round_to(value: f64, places: i32) -> f64 {
    let multiplier = 10f64.powi(places);
    (value * multiplier).round() / multiplier
}

pub fn string_to_file(string: &String, filename: impl AsRef<Path>) {
    let mut file = File::create(filename).expect("Could not create file.");

    file.write_all(string.as_bytes())
        .expect("Could not write to file.");
}
