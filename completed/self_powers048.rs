const MODULO: u64 = 10000000000;

trait Crypto {
	fn mod_pow(&self,e: u64, m: u64) -> u64;
}

impl Crypto for u64 {
	fn mod_pow(&self,e: u64, m: u64) -> u64
	{
		let mut answer = 1;
		for _ in 0..e {
			answer = (answer * self) % m;
		}
		answer
	}
}

fn main() {
	let mut answer = 0;
	for i in (1 as u64)..1000+1 {
		answer += i.mod_pow(i,MODULO);
		answer %= MODULO;
	}
	println!("The answer is: {}",answer);
}
