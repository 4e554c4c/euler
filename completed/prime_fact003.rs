use std::io;
use std::vec;


fn factor(num: i64) -> i64
{
	//returs first prime factor of num. (if num is prime, return 1)
	for i in (2..num).take_while(|i| i * i < num)
	{
		if num % i == 0
		{ 
			return i; 
		}
	}
	num
}
fn prime_factors() 
{
	let mut vec = Vec::new();
	loop
	{
		
		println!("Please input the number you would like factored.");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Failed to read guess.");

		let mut guess : i64 = guess.trim().parse()
			.expect("Please type a number!");
		
		loop
		{
			let factor = factor(guess);
			vec.push(factor);
			guess /= factor;
			if guess == 1
			{
				break;
			}
		}
		println!("{:?}",vec);
	}
}

fn main(){
	prime_factors();
}
