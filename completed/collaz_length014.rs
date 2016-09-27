use std::io;

fn collaz_length(num : u64) -> u64
{
	let mut num = num;
	let mut length = 0;

	while num != 1{
		if num % 2 == 0{
			num /= 2;
		}else{
			num *= 3;
			num += 1;
		}
		length += 1;
	}
	length
}
fn main()
{
	let limit = 1000000;
	let mut max = 0;
	let mut answer = 0;

	for i in 1..limit {
		let tmp = collaz_length(i);
		if tmp > max {
			max = tmp;
			answer = i;
		}
	}
	println!("The largest collaz sequence is {}, the number that produces it is {}",max,answer);
}
