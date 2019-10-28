// bring io library (from the std library) into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // :: indicates that new is an associated function of the String type
        // An associated function is implemented on a type. 
        // Some languages call this a static method
        let mut guess = String::new();

        // & indicates that the argument is a reference,
        // which gives us a way to let multiple sections of code access 
        // a single piece of data without needing to 
        // copy that data into memory multiple times

        // &mut indicates that the reference is mutable
        io::stdin().read_line(&mut guess)
            // read_line returns a Result type
            // result types are enums -- a type that can have a fixed set of values (known as the enum's 'variants')
            // purpose of Result type is to encode error handling info. 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Ordering is an enum, with variants Less, Greater and Equal
        match guess.cmp(&secret_number) {
            // the three lines below are 'arms',
            // which consist of a pattern and the code that should be run if the pattern is matched
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
