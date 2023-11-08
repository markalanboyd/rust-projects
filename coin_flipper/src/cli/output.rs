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
