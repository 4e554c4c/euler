fn is_sum_digit_power(x: u64, pow: u32) -> bool
{
	let mut sum = 0;
	{
		let mut x = x;
		while x != 0 {
			sum += (x%10).pow(pow);
			x /= 10;
		}
	}
	x == sum
}
fn main()
{
	let pow = 5;
	let mut answer = 0;
	for i in 10..10000000 {
		if is_sum_digit_power(i,pow) {
			println!("Power digit sum found! {}",i);
			answer += i;
		}
	}
	println!("The answer is: {}",answer);
}
