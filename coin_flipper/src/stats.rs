use std::collections::HashMap;

use crate::utils::round_to;
use num_format::{Locale, ToFormattedString};

pub fn display_results(flip_sequence: &String) {
    let mut char_counter: i32 = 1;
    for flip_result in flip_sequence.chars() {
        print!("{}", flip_result);
        char_counter += 1;
        if char_counter % 80 == 0 {
            print!("...\n");
            println!("See 'results.txt' for more");
            return;
        }
    }
    println!(); // add an extra line if less than 80
}

struct CoinFlipStats {
    heads_formatted: String,
    tails_formatted: String,
    total_difference_formatted: String,
    percent_heads_rounded: f64,
    percent_tails_rounded: f64,
    percent_difference_rounded: f64,
}

impl CoinFlipStats {
    fn new(times: i32, heads: i32, tails: i32) -> Self {
        let heads_formatted: String = heads.to_formatted_string(&Locale::en);
        let tails_formatted: String = tails.to_formatted_string(&Locale::en);
        let total_difference: i32 = (heads - tails).abs();
        let total_difference_formatted = total_difference.to_formatted_string(&Locale::en);

        let percent_heads: f64 = (heads as f64 / times as f64) * 100.0;
        let percent_tails: f64 = 100.0 - percent_heads;
        let percent_heads_rounded: f64 = round_to(percent_heads, 5);
        let percent_tails_rounded: f64 = round_to(percent_tails, 5);

        let percent_difference: f64 = (percent_heads - percent_tails).abs();
        let percent_difference_rounded: f64 = round_to(percent_difference, 5);

        Self {
            heads_formatted,
            tails_formatted,
            total_difference_formatted,
            percent_heads_rounded,
            percent_tails_rounded,
            percent_difference_rounded,
        }
    }
}

pub fn display_stats(times: i32, heads: i32, tails: i32, duration: std::time::Duration) {
    let stats = CoinFlipStats::new(times, heads, tails);

    print!("\n\n");
    println!("::Stats::");
    println!("Number");
    println!("Heads: {}", stats.heads_formatted);
    println!("Tails: {}", stats.tails_formatted);
    println!("Difference: {}", stats.total_difference_formatted);
    println!();
    println!("Percent");
    println!("Heads: {}%", stats.percent_heads_rounded);
    println!("Tails: {}%", stats.percent_tails_rounded);
    println!("Difference: {}%", stats.percent_difference_rounded);
    println!();

    println!("Execution time: {:?}", duration);

    println!("\n");
}

pub fn get_most_chars_in_a_row(string: &str) -> HashMap<char, i32> {
    let mut count: i32 = 1;
    let mut char_streaks: HashMap<char, i32> = HashMap::new();
    let mut prev_char: Option<char> = None;

    for current_char in string.chars() {
        if let Some(ch) = prev_char {
            if ch == current_char {
                count += 1;
            } else {
                update_streaks(&mut char_streaks, ch, count);
                count = 1;
            }
        }
        prev_char = Some(current_char);
    }

    if let Some(ch) = prev_char {
        update_streaks(&mut char_streaks, ch, count);
    }

    char_streaks
}

fn update_streaks(char_streaks: &mut HashMap<char, i32>, ch: char, count: i32) {
    let high_count = char_streaks.entry(ch).or_insert(0);
    *high_count = (*high_count).max(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_in_a_row() {
        let test_string = "aabbaaccdddbbaaa".to_string();
        let mut expected = HashMap::new();
        expected.insert('a', 3);
        expected.insert('b', 2);
        expected.insert('c', 2);
        expected.insert('d', 3);

        let result = get_most_chars_in_a_row(&test_string);

        assert_eq!(result, expected);
    }
}
