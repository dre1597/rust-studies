use std::io;

mod lib;

fn main() {
    let mut input = String::new();

    println!("Enter the seconds: ");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let seconds: Result<i32, _> = input.trim().parse();

    match seconds {
        Ok(n) => {
            let (days, hours, minutes, seconds) = lib::convert_seconds(n);
            println!("{} seconds is equal to", n);
            println!("{}d, {}h, {}m, {}s", days, hours, minutes, seconds);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
