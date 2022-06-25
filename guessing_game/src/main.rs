use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number: u8 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("What's your guess?");
        let mut input = String::new();

        stdin().read_line(&mut input).expect("Failed to read line");

        let guess: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input an integer");
                continue;
            }
        };
        println!("You guessed {}", guess);

        // Without Ordering Enum
        // if guess < secret_number {
        //     println!("Too Small!");
        // } else if guess > secret_number {
        //     println!("Too Big!");
        // } else {
        //     println!("You Win!");
        //     break;
        // }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
