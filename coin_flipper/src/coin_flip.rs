use rand;

enum CoinState {
    Heads,
    Tails,
}

pub struct CoinFlipSession {
    pub heads: i32,
    pub tails: i32,
    pub flip_sequence: String,
    pub duration: std::time::Duration,
}

impl CoinFlipSession {
    pub fn new() -> Self {
        CoinFlipSession {
            heads: 0,
            tails: 0,
            flip_sequence: String::new(),
            duration: std::time::Duration::new(0, 0),
        }
    }

    pub fn flip_coins(&mut self, times: i32) {
        let function_timer: std::time::Instant = std::time::Instant::now();
        for _ in 0..times {
            let flip: CoinState = if rand::random() {
                CoinState::Heads
            } else {
                CoinState::Tails
            };
            match flip {
                CoinState::Heads => {
                    self.heads += 1;
                    self.flip_sequence.push('H');
                }
                CoinState::Tails => {
                    self.tails += 1;
                    self.flip_sequence.push('T');
                }
            }
        }
        self.duration = function_timer.elapsed()
    }
}
