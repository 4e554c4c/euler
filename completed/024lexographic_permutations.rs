fn factorial(x: u8) -> u64
{
	if x == 1 {
		return 1;
	}
	let x : u64 = x as u64;
	x * factorial((x-1) as u8) 
}
fn unrank_lexographic_permutation(b: u8, n: u64) -> Vec<u8>
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
			unrank_lexographic_permutation(b - 1, n % perms);
		for i in remaining {
			result.push(num[i as usize]);
		}
	}
	result
}
fn main()
{
	let mut answer = 0;
	for i in unrank_lexographic_permutation(10,999999){
		answer *= 10;
		answer += i as u32;
	}
	println!("The answer is: {}",answer);
}
