extern crate bit_vec;
use bit_vec::BitVec;

const MAX: usize = 1000000;

fn main()
{
	let primes = {
		let mut bv = BitVec::from_elem(MAX,true);
		bv.set(0,false);bv.set(1,false);

		for i in (2 as usize..).take_while(|x| x.pow(2) <= MAX) {
			if bv[i]{
				for j in (i..).map(|x| x * i)
							  .take_while(|x| *x < MAX){
					bv.set(j,false); }
			}
		}
		bv
	};

	let mut answer = 0;
	let mut max = 0;
	for j in (1..MAX).filter(|&x| primes[x]) {
		for k in (j..MAX).filter(|&x| primes[x]).enumerate() {
			if k.0 <= max {
				continue;
			}

			let sum = (j..k.1 + 1).filter(|&x| primes[x])
				.fold(0,|acc, x| acc + x);

			if sum > MAX {
				break;
			}

			if primes[sum] {
				max = k.0;
				answer = sum;
			}
		}
	}
	println!("The answer is: {}",answer);
}
