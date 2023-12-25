use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number: ");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let seconds: Result<i32, _> = input.trim().parse();

    match seconds {
        Ok(n) => {
            println!("You entered {} seconds", n);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
