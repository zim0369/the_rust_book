// ANCHOR: all
use rand::{thread_rng, Rng};
use std::cmp::Ordering::{Equal, Greater, Less};

fn main() {
    let random: u32 = thread_rng().gen_range(1..=9);
    println!("Secret number: {random}");

    loop {
        println!("Guess the number: ");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).unwrap();
        match guess.trim().parse::<u32>().unwrap().cmp(&random) {
            Greater => println!("Too large"),
            Less => println!("Too small"),
            Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
// ANCHOR: all
