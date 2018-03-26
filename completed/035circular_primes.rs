extern crate bit_vec;
use bit_vec::BitVec;

const MAX: usize = 1000000;

fn main()
{
	//prime sieve
	let primes = {
		let mut bv = BitVec::from_elem(MAX,true);
		bv.set(0,false);bv.set(1,false);

		for i in (2 as usize..).take_while(|x| x.pow(2) <= MAX) {
			if bv[i]{
				for j in (i..).map(|x| x * i)
							  .take_while(|x| *x < MAX){
					bv.set(j,false);
				}
			}
		}
		bv
	};
	println!("Primes found!");

	let mut answer = 0;
	for i in 2..MAX {
		let mut num = Vec::new();
		let mut state = true;

		let mut temp = i;
		while temp != 0 {
			num.push(temp%10);
			temp /= 10;
		}
		let len = num.len();

		for j in 0..len {
			let mut rotation = 0;
			for k in 0..len {
				rotation *= 10;
				rotation += num[(k + j) % len];
			}
			if primes[rotation] == false {
				state = false;
				break;
			}
		}
		if state {
			println!("Rotational prime found! {}",i);
			answer += 1;
		}
	}
	println!("The answer is: {}",answer);
}
