use std::cmp::Ordering; // Ordering type 사용
use std::io; // input/output library
use rand::Rng; // random library

fn main() {
    println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end -> start 이상 end 이하
	
	// println!("The secret number is {secret_number}");
	
	loop {
		println!("Please input your guess.");
	
		// let: immutable
		// let mut: muttable 
		// String::new() -> 새로운 빈 문자열 생성
		let mut guess = String::new();

		// user input
		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line"); // 예외처리

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			},
		}
	}
}
