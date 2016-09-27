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
	for i in 10..MAX {
		let mut num = Vec::new();
		let mut state = true;

		let mut temp = i;
		while temp != 0 {
			num.push(temp%10);
			temp /= 10;
		}
		let len = num.len();

		for j in 0..len {
			let mut trunication1 = 0;
			for k in (0..len-j).rev() {
				trunication1 *= 10;
				trunication1 += num[k];
			}
			let mut trunication2 = 0;
			for k in (j..len).rev() {
				trunication2 *= 10;
				trunication2 += num[k];
			}
			/*
			 *println!("  Trunacation tried:{}",trunication1);
			 *println!("  Trunacation tried:{}",trunication2);
			*/
			 if primes[trunication1] == false||primes[trunication2] == false{
				state = false;
				break;
			}
		}
		if state {
			println!("trunicational prime found! {}",i);
			answer += i;
		}
	}
	println!("The answer is: {}",answer);
}
