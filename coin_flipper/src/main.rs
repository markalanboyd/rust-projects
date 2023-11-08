use num_format::{Locale, ToFormattedString};

use std::time::Instant;

mod utils;
use utils::file_utils::string_to_file;
use utils::random_utils::flip_coin;
use utils::utils::round_to;

mod cli;
use cli::input::prompt_for_flips;
use cli::output::display_results;

fn main() {
    loop {
        let times = prompt_for_flips();

        println!("\nFlipping coin {times} times...\n");

        let start = Instant::now();
        let (results, heads, tails) = flip_coin(times);
        let duration = start.elapsed();

        display_results(&results);

        string_to_file(&results, "results.txt");

        // TODO Break into function
        let heads_fmt: String = heads.to_formatted_string(&Locale::en);
        let tails_fmt: String = tails.to_formatted_string(&Locale::en);
        let percent_heads: f64 = (heads as f64 / times as f64) * 100.0;
        let percent_tails: f64 = 100.0 - percent_heads;
        let percent_heads_rounded: f64 = round_to(percent_heads, 5);
        let percent_tails_rounded: f64 = round_to(percent_tails, 5);

        let total_difference: i32 = (heads as i32 - tails as i32).abs();
        let percent_difference: f64 = (percent_heads - percent_tails).abs();
        let percent_difference_rounded: f64 = round_to(percent_difference, 5);
        let total_diff_fmt: String = total_difference.to_formatted_string(&Locale::en);

        print!("\n\n");
        println!("::Stats::");
        println!("Number");
        println!("Heads: {}", heads_fmt);
        println!("Tails: {}", tails_fmt);
        println!("Difference: {}", total_diff_fmt);
        println!();
        println!("Percent");
        println!("Heads: {}%", percent_heads_rounded);
        println!("Tails: {}%", percent_tails_rounded);
        println!("Difference: {}%", percent_difference_rounded);
        println!();

        println!("Execution time: {:?}", duration);

        println!("\n");
    }
}
