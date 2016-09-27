use std::vec;

fn factorial(x: u64) -> u64
{
	if x <= 1 {return 1};
	factorial(x-1) * x
}

fn sum_of_factorial_digits(x: u64) -> u64 
{
	let mut x = x;
	let mut digits = Vec::new();
	while x != 0 {
		digits.push((x%10));
		x /= 10;
	}
	digits.iter().map(|x| factorial(*x)).fold(0,|acc, x| acc + x)
}

fn main() {
	let mut answer = 3..1000000
		.collect(|x| sum_of_factorial_digits(x) == x)
		.fold(0,|acc, x| acc + x);
	println!("The answer is: {}",answer);
}
