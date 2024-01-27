use std::io;

mod easy;

fn main() {
    println!("Welcome!");
    let mut input = String::new();

    println!("Enter palindrome number:");

    io::stdin().read_line(&mut input).expect("Error reading");

    let input: i32 = match input.trim().parse() {
        Ok(result) => result,
        Err(_) => panic!("Input is not a number."),
    };

    let result = easy::palindrome::is_palindrome(input);
    println!("{result}");
}
