fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");

    // Sleep for 5 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));

    loop {
        // Generate random number
        let random_number = rand::random::<u8>() % 4;

        // Add emoji to the end based on the random number
        let emoji = match random_number {
            0 => "🦀",
            1 => "🦀🦀",
            2 => "🦀🦀🦀",
            _ => "🦀🦀🦀🦀",
        };

        // Print the emoji and "I'm a Rustacean!"
        println!("I'm a Rustacean! {}", emoji);
    }
}
