fn is_prime(num: i64) -> bool
{
	if num <= 1 { return false; }
	if num <  4 { return true; }
	if num %  2 == 0 { return false; }
	if num <  9 { return true; }
	if num %  3 == 0 { return false; }

	for i in (1..).map(|x| x * 6).take_while(|x| x*x <= num) {
		if num % (i - 1) == 0 || num % (i + 1) == 0{
			return false;
		}
	}
	true
}
fn main() 
{
	let limit = 1000;
	let mut max = 0;
	let mut answer = (1,1);

	for a in (0..)
		.map(|x| (x >> 1) ^ -(x & 1))
		.take_while(|x| x <= &limit) {

		for b in (0..)
			.map(|x| (x >> 1) ^ -(x & 1))
			.take_while(|x| x <= &limit) {

			for n in 0.. {
				if !is_prime((n * n) + (a * n) + b){
					if n >= max {
						max = n;
						answer.0 = a;
						answer.1 = b;
					 println!("n^2 + {}n + {} has {} primes",
							  answer.0,answer.1,n);
					}
					break;
				}
			}
		}
	}
	println!("The answer is: {} x {} = {}",
			 answer.0,answer.1,answer.0*answer.1);
}
