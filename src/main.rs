#[warn(non_snake_case)]
extern crate rand;
use rand::Rng;

fn pass_generator(){

	let string_pass_chars="qQwWeErRtTyYuUiIoOpPaAsSdDfFgGhHjJkKlLzZxXcCvVbBnNmM1234567890!@#";
	let mut pass= String::new();
	while(pass.len()<13){
		let number=rand::thread_rng().gen_range(0,65);
		let char_=string_pass_chars.chars().nth(number).unwrap();
		pass.push(char_);
	}	
	println!("{}",pass);
}

fn main(){

		println!("You password generated: ");
		pass_generator();

}