const MAX: usize = 1000000;

fn digits(num: u64) -> Vec<u8>
{
	let mut num = num;
	let mut digits = Vec::new();
	while num != 0 {
		digits.push((num%10) as u8);
		num /= 10;
	}
	digits.reverse();
	digits
}
fn main() {
	let mut constant = Vec::new();
	for i in 1.. {
		if constant.len() >= MAX {
			break;
		}
		
		constant.append(&mut digits(i));
	}
	let mut answer = 1;
	for i in (0 as u32..6+1).map(|x| (10 as usize).pow(x)) {
		println!("constant[{}] = {}",i,constant[i-1]);
		answer *= constant[i-1] as u64;
	}
	println!("The answer is: {}",answer);
}
