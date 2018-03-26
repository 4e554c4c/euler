use std::io;

fn main()
{
	println!("Please input the number you would like analized.");

	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
		.expect("Please type a number!");

	let mut bytes = guess.into_bytes();
	
	println!("The number is {} digits", bytes.len());
	//convert from ascii to dec
	for i in 0..bytes.len(){
		assert!(bytes[i] <= '9' as u8 && bytes[i] >= '0' as u8); // ascii '9' and '0' respectively
		bytes[i] = bytes[i] - '0' as u8;
	}
	
	//comptute
	let mut max	 : u64 = 0;
	let mut product : u64 = 1;
	for i in 0..bytes.len() - 13 {
		for j in 0..13 {
			product *= bytes[i+j] as u64;
		}
		if product >= max{
			max = product;
		}
	}
	println!("The max product is {}",max);
}
