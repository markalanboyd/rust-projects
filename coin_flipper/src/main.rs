// TODO Add a method that looks through the string you create and looks for the longest streak
// TODO Store the streak in a high score file - make it so that it adds your 3 initials
// TODO Add config options for how the data is returned - try converting it to interesting types

mod cli;
mod coin_flip;
mod stats;
mod utils;

use std::collections::HashMap;

use cli::prompt_for_flips;
use coin_flip::CoinFlipSession;
use stats::{display_results, display_stats, get_most_chars_in_a_row};
use utils::string_to_file;

fn main() {
    loop {
        let times: i32 = prompt_for_flips();
        let mut flip_results = CoinFlipSession::new();
        flip_results.flip_coins(times);

        display_results(&flip_results.flip_sequence);
        display_stats(
            times,
            flip_results.heads,
            flip_results.tails,
            flip_results.duration,
        );

        let char_streaks: HashMap<char, i32> = get_most_chars_in_a_row(&flip_results.flip_sequence);
        let heads_streak: &i32 = char_streaks.get(&'H').unwrap_or(&0);
        let tails_streak: &i32 = char_streaks.get(&'T').unwrap_or(&0);
        println!("Most heads in a row: {}", heads_streak);
        println!("Most tails in a row: {}", tails_streak);

        string_to_file(&flip_results.flip_sequence, "results.txt");
    }
}
