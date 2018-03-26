extern crate bit_vec;
use bit_vec::BitVec;

fn is_pandigital(num: u64) -> bool
{
	let mut num = num;
	let mut digits = Vec::new();
	while num != 0 {
		digits.push(num%10);
		num /= 10;
	}
	digits.sort();
	digits == {
		let mut nums = Vec::new();
		for i in 1..digits.len() as u64+1 {
			nums.push(i);
		}
		nums
	}
}
fn main()
{
	let mut answer = 0;
	let primes = {
		let mut bv = BitVec::from_elem(10000000,true);
		bv.set(0,false);bv.set(1,false);

		for i in (2 as usize..).take_while(|x| x.pow(2) <= 10000000) {
			if bv[i]{
				for j in (i..).map(|x| x * i)
					.take_while(|x| *x < 10000000){
					bv.set(j,false);
				}
			}
		}
		bv
	};
	for i in 0..10000000 {
		if primes[i] {
			if is_pandigital(i as u64){
				println!("Pandigital prime found! {}",i);
				answer = i;
			}
		}
	}
	println!("The answer is: {}",answer);
}
