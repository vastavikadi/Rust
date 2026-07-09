use rand::Rng;
use std::cmp::Ordering;
use std::io; //input/output lib from the standard lib of rust

// fn main() {
//     //main function is the entry point into the program
//     println!("Guess the number!");
//     let secret_number = gen_random();

//     println!("Please enter your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read the guess.");

//     println!("You guessed: {guess}");

//     let mut guessed_num = match guess.trim().parse::<i32>() {
//         Ok(num) => num,
//         Err(e) => {
//             println!("Error: {}", e);
//             return;
//         }
//     };

//     let mut res = check_the_guess(secret_number, guessed_num);
//     while res {
//         println!("Please enter your guess.");
//         guess.clear();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read the guess.");

//         guessed_num = match guess.trim().parse::<i32>() {
//             Ok(num) => num,
//             Err(e) =>{
//                 println!("Error: {}", e);
//                 return;
//             }
//         };
//         res = check_the_guess(secret_number, guessed_num);
//     }
// }

// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.

// fn gen_random() -> i32 {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("This is the secret number: {secret_number}");
//     return secret_number;
// }

// fn check_the_guess(secret_num: i32, guess: i32) -> bool {
//     if secret_num > guess {
//         println!("GO Higher!");
//         return true;
//     } else if secret_num < guess {
//         println!("GO Lower!");
//         return true;
//     } else {
//         println!("You guessed it right! YooHoo!, you won. Congrats!");
//         return false;
//     }
// }

fn main() {
    println!("Guess the number!");
    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        //quit will quit the game, so will entering any other non-number input.
        println!("Please enter your input");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,//to ignore the input of any numeric value,and again asks for another right guess
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
