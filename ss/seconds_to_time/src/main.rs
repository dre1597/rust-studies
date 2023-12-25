use std::io;

fn convert_seconds(seconds: i32) -> (u64, u64, u64, u64) {
    let days = seconds / (24 * 3600);
    let mut remain_seconds = seconds % (24 * 3600);

    let hours = remain_seconds / 3600;
    remain_seconds %= 3600;

    let minutes = remain_seconds / 60;
    remain_seconds %= 60;

    (days as u64, hours as u64, minutes as u64, remain_seconds as u64)
}

fn main() {
    let mut input = String::new();

    println!("Enter the seconds: ");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let seconds: Result<i32, _> = input.trim().parse();

    match seconds {
        Ok(n) => {
            let (days, hours, minutes, seconds) = convert_seconds(n);
            println!("{} seconds is equal to", n);
            println!("{}d, {}h, {}m, {}s", days, hours, minutes, seconds);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
