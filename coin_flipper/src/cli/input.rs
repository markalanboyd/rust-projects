use std::io::{self, Write};

pub fn prompt_for_flips() -> i32 {
    println!("----------------------------");
    println!("How many times to flip coin?\n");
    loop {
        print!("Times: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut times: String = String::new();

        io::stdin()
            .read_line(&mut times)
            .expect("Failed to read line.");

        let times: i32 = match times.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("\nError: Please enter a positive 32-bit number.\n");
                continue;
            }
        };

        println!("\nFlipping coin {times} times...\n");
        return times;
    }
}
