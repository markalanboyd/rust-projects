use crate::utils::utils::round_to;
use num_format::{Locale, ToFormattedString};

pub struct CoinFlipStats {
    pub heads_formatted: String,
    pub tails_formatted: String,
    pub total_difference_formatted: String,
    pub percent_heads_rounded: f64,
    pub percent_tails_rounded: f64,
    pub percent_difference_rounded: f64,
}

impl CoinFlipStats {
    pub fn new(times: i32, heads: i32, tails: i32) -> Self {
        let heads_formatted = heads.to_formatted_string(&Locale::en);
        let tails_formatted = tails.to_formatted_string(&Locale::en);
        let total_difference = (heads - tails).abs();
        let total_difference_formatted = total_difference.to_formatted_string(&Locale::en);

        let percent_heads = (heads as f64 / times as f64) * 100.0;
        let percent_tails = 100.0 - percent_heads;
        let percent_heads_rounded = round_to(percent_heads, 5);
        let percent_tails_rounded = round_to(percent_tails, 5);

        let percent_difference = (percent_heads - percent_tails).abs();
        let percent_difference_rounded = round_to(percent_difference, 5);

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
