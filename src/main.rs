
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // println!("Guess the number!");
    // let secrect_number = rand::thread_rng().gen_range(1, 101);
    // // println!("The secrect number is {}", secrect_number);
    // loop {
    //     println!("Please input your guess");
    //     let mut guess = String::new();
    //     io::stdin().read_line(&mut guess).expect("Failed to read libe");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("You guessed: {}", guess);
    
    //     match guess.cmp(&secrect_number) {
    //         Ordering::Less => println!("Too samll"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win");
    //             break;
    //         }
    //     }
    // }
    clone();
}

fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}
