use std::{env, thread, process};
use crossterm::event::{Event, KeyCode, KeyEvent, read};
use crossterm::terminal::{self};

fn main() {
    terminal::enable_raw_mode().unwrap();

    thread::Builder::new().name("inputHandler".to_string()).spawn(|| {
        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {code: KeyCode::Char('q'), ..}) => {
                    terminal::disable_raw_mode().unwrap();
                    process::exit(0);
                }
                _ => {},
            }
        }
    }).ok();

    let mut n:u128 = 0;
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1].chars().all(char::is_numeric) {
            n = args[1].parse::<u128>().unwrap();
        } else {
            println!("ERROR 0x00: Invalid argument. The first argument must be a numeric value. The given value was \"{}\".", args[1]);
            process::exit(1);
        }
    }
    loop {
        if is_prime(n) {
            println!("{} is prime!", n);
        }
        n += 1;
    }
}

fn is_prime(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    return true;
}