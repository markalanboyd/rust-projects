use rand::Rng;

pub fn flip_coin(times: i32) -> (String, i32, i32) {
    let mut rng = rand::thread_rng();
    let mut results: String = String::new();
    let mut heads_count: i32 = 0;
    let mut tails_count: i32 = 0;

    for _ in 0..times {
        let coin_flip: bool = rng.gen_bool(0.5);
        if coin_flip {
            heads_count += 1;
            results.push('H');
        } else {
            tails_count += 1;
            results.push('T');
        }
    }
    return (results, heads_count, tails_count);
}
