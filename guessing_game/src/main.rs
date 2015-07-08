extern crate rand;

use std::io;
use std::cmp::Ordering;

fn main() {
    let secret = (rand::random::<u32>() % 100) + 1;

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut input_stream = io::stdin();
        let mut buffer = String::new();

        input_stream.read_line(&mut buffer)
                    .ok()
                    .expect("There was an error.");
        
        let input_num: Option<u32> = buffer.trim().parse().ok();

        let num = match input_num {
            Some(num) => num,
            None => {
                println!("please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", buffer);

        match cmp(num, secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    println!("The secret number is: {}", secret);
}

fn cmp(a: u32, b: u32) -> Ordering {
    if a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else { Ordering::Equal }
}