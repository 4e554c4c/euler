fn is_prime(num: i64) -> bool
{
	//returs first prime factor of num. (if num is prime, return 1)
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
	let mut range = 2..;
	let mut prime = 2;
	for i in 0..10002 {
		loop{
			match range.next(){
				Some(x) => {
					if is_prime(x) {
						prime = x;
						break;
					}
				},
				None => {panic!();}
			}
		}
		println!("Prime {}: {}",i,prime);
	}
}
