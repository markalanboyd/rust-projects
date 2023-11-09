// TODO Add a method that looks through the string you create and looks for the longest streak
// TODO Store the streak in a high score file - make it so that it adds your 3 initials
// TODO Add config options for how the data is returned - try converting it to interesting types

mod utils;
use utils::file_utils::string_to_file;
use utils::random_utils::flip_coin;
use utils::utils::time_execution;

mod cli;
use cli::input::prompt_for_flips;
use cli::output::{display_results, display_stats};

fn main() {
    loop {
        let times = prompt_for_flips();

        let ((results, heads, tails), duration) = time_execution(|| flip_coin(times));

        display_results(&results);
        display_stats(times, heads, tails, duration);

        string_to_file(&results, "results.txt");
    }
}
