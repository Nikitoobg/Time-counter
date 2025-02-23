use std::thread;
use std::time::Duration;

fn main() {
    let mut seconds = 0;
    let mut minutes = 0;
    let mut hours = 0;
    let mut days = 0;
    loop {
        println!("{} days, {} hours, {} minutes, {} seconds",days, hours, minutes, seconds );
        seconds += 1;
        thread::sleep(Duration::from_millis(1000));
        lines();

        if seconds == 60 {
            minutes += 1;
            seconds = 0
        }

        if minutes == 60 {
            hours += 1;
            minutes = 0
        }

        if hours == 24 {
            days += 1;
            hours = 0
        }
    }
}

fn lines() {
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
}
