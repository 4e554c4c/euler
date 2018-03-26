fn is_prime(num: u32) -> bool
{
	//returs first prime factor of num. (if num is prime, return false)
	for i in 2..num ^ (1/2)
	{
		if num % i == 0
		{ 
			return false;
		}
	}
	true
}
fn main() {
	let	 upto: u32 = 2000000;
	let mut sum : u64 = 0;
	
	for i in 2..upto {
		if is_prime(i){
			sum += i as u64;
		}
	}
	println!("The sum of primes from 2 to {} is {}",upto,sum);
}
