extern crate bit_vec;
use bit_vec::BitVec;

fn main()
{
	let mut answer = 0;
	let primes = {
		let mut bv = BitVec::from_elem(10000,true);
		bv.set(0,false);bv.set(1,false);

		for i in (2 as usize..).take_while(|x| x.pow(2) <= 10000) {
			if bv[i]{
				for j in (i..).map(|x| x * i)
					.take_while(|x| *x < 10000){
					bv.set(j,false);
				}
			}
		}
		bv
	};
	'outer: for i in (6..10000).filter(|&x| !primes[x] && x % 2 ==  1) {
		for j in (0..).filter(|&x| primes[x]) {
			if i < j {
				answer = i;
				break 'outer;
			}
			for k in (0..).map(|x| 2 * (x * x)).take_while(|&x| x +j <= i) {
				if j + k == i {
					println!("{} = {} * {}",i,j,k);
					continue 'outer;
				}
			}
		}
	}
	println!("The answer is: {}",answer);
}
