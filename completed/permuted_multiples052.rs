fn is_permutation(a: usize, b: usize) -> bool
{
	let mut a = a;
	let mut b = b;
	let mut a_digits = Vec::new();
	let mut b_digits = Vec::new();
	while a != 0 {
		a_digits.push(a%10);
		a /= 10;
	}
	while b != 0 {
		b_digits.push(b%10);
		b /= 10;
	}
	a_digits.sort();
	b_digits.sort();
	a_digits == b_digits
}

fn main()
{
	println!("The answer is: {:?}",(1..)
		.filter(|&x|
				is_permutation(x, x * 2) &&
				is_permutation(x, x * 3) &&
				is_permutation(x, x * 4) &&
				is_permutation(x, x * 5) &&
				is_permutation(x, x * 6))
		.next().unwrap());
}
