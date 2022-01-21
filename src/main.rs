use rand::Rng;
use std::io::BufRead;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..101);

    println!("Guess a number between 1 & 100 ");

    for line in std::io::stdin().lock().lines() {
        let parsed = line.ok().as_deref().map(str::parse::<i64>);
        if let Some(Ok(guess)) = parsed {
            match guess {
                _ if guess < random => println!("too low"),
                _ if guess > random => println!("too high"),
                _ => {
                    println!("That's right");
                    break;
                }
            }
        }
    }
}
