fn sum_divisors(num : u32) -> u32 
{
	let mut sum = 0;
	for i in 1..num {
		if num % i == 0 {
			sum += i;
		}
	}
	sum
}
fn main()
{
	let mut sum = 0;
	
	for i in 0..10000 {
		let friend = sum_divisors(i);
		if i != friend && i == sum_divisors(friend) {
			sum += i;
			println!("Amicable numbers {} and {} found",i,friend);
		}
	}
	println!("The answer is {}",sum);
}
