
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the guess game");

    loop {       
        let secret_num = rand::thread_rng().gen_range(1, 101);
        // println!("The generate secret number is: {}", secret_num);

        let mut guess = String::new();
        println!("Please input your number: ");
        io::stdin().read_line(&mut guess).expect("Invalid value entered");
        println!("The entered value is: {}", guess);
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small!!!"),
            Ordering::Greater => println!("Too Big!!!"),
            Ordering::Equal => {
                println!("You got it :)");
                break;
            }
        }
    }
}
