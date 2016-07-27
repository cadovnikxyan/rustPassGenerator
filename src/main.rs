#[warn(non_snake_case)]
extern crate rand;
use rand::Rng;

fn pass_generator(){

	let  array="qQwWeErRtTyYuUiIoOpPaAsSdDfFgGhHjJkKlLzZxXcCvVbBnNmM1234567890!@#".chars();

	let mut pass=String::new();

	for x in array{
		let number=rand::thread_rng().gen_range(0,60) as usize;
		pass.push(Some(array.nth(number).unwrap()));
	}	
	println!("{}",pass);
}

fn main(){

		pass_generator();

}