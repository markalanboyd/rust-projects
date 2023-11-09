use crate::utils::coin_flip_stats::CoinFlipStats;

pub fn display_results(results: &String) {
    let mut counter = 0;
    for flip_result in results.chars() {
        print!("{}", flip_result);
        counter += 1;
        if counter % 100 == 0 {
            print!("...\n");
            println!("See 'results.txt' for more");
            break;
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
