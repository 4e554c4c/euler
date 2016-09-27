fn is_palendrome(num: u64,base: u64) -> bool
{
	let mut mun = 0;
	{
		let mut num = num;
		while num !=0 
		{
			mun *= base;
			mun += num % base;
			num /= base;
		}
	}
	mun == num
}
fn main()
{
	println!("The answer is {}",(0..1000000)
		.filter(|x|is_palendrome(*x,10)&&is_palendrome(*x,2))
		.fold(0,|acc,x| acc + x));
}
