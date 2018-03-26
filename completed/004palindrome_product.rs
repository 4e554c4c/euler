use std::io;
fn is_palendrome(num: i32) -> bool
{
	let mut mun = 0;
	{
		let mut num = num;
		while num !=0 
		{
			mun *= 10;
			mun += num % 10;
			num /= 10;
		}
	}
	if mun == num{
		return true;
	}
	false
}
fn main()
{
	let mut max = 0;
	for i in 100..1000{
		for j in i..1000{
			let product = i*j;
			if is_palendrome(product) {
				if product > max {
					max = product;
				}
			}
		}
	}
	println!("Max: {}", max);
}
