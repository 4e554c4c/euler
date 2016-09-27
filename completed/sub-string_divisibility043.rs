fn factorial(x: u8) -> u64
{
	if x == 1 {
		return 1;
	}
	let x : u64 = x as u64;
	x * factorial((x-1) as u8) 
}
fn unrank_lexographic_permutation(n: u64,b: u8) -> Vec<u8>
{
	if b == 1 || n == 0 {
		return (0..b).collect();
	}

	let mut num: Vec<_> = (0..b).collect();
	let mut result	  = Vec::new();

	let perms: u64	  = factorial(b-1);
	result.push(num.remove((n / perms) as usize));

	{
		let remaining = 
			unrank_lexographic_permutation(n % perms, b - 1);
		for i in remaining {
			result.push(num[i as usize]);
		}
	}
	result
}
fn substring(num: u64, bound: (usize,usize)) -> u64
{
	let mut num = num;
	let mut digits = Vec::new();
	while num != 0 {
		digits.push(num%10);
		num /= 10;
	}
	assert!(bound.0 > 0);
	while bound.1 > digits.len() {
		digits.push(0);
	}

	let mut substring = 0;
	digits.reverse();

	for i in digits.into_iter().enumerate()
	.filter(|&x| x.0 + 1 >= bound.0 && x.0 + 1<= bound.1) {
		substring *= 10;
		substring += i.1;
	}
	substring
}
fn main() {
	println!("The answer is: {}",
		(0..factorial(10)).map(|x|
			unrank_lexographic_permutation(x,10).into_iter()
			.fold(0,|acc, x| (acc * 10) + x as u64)
		)
		.filter(|&x|substring(x,(2, 4)) %  2 == 0
				 && substring(x,(3, 5)) %  3 == 0
				 && substring(x,(4, 6)) %  5 == 0
				 && substring(x,(5, 7)) %  7 == 0
				 && substring(x,(6, 8)) % 11 == 0
				 && substring(x,(7, 9)) % 13 == 0
				 && substring(x,(8,10)) % 17 == 0)
		.fold(0,|acc, x| acc + x));
}
