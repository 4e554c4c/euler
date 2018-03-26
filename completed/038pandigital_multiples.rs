fn is_pandigital(nums: &Vec<u64>) -> bool
{
	let mut digits = Vec::new();
	for i in nums {
		let mut i = *i;
		while i != 0 {
			digits.push(i%10);
			i /= 10;
		}
	}
	digits.sort();
	digits == vec![1,2,3,4,5,6,7,8,9]
}
fn main()
{
	let mut answer = 0;
	for i in 1..100000 {
		for j in 2..10 {
			let mut products = Vec::new();
			for k in 1..j{
				products.push(i*k);
			}
			if is_pandigital(&products) {
				let mut concatenated_product = 0;
				let mut last_product = 0;
				//The concat product part doesn't work yet
				for product in products {
					concatenated_product *= (10 as u64).pow(9/(j-1) as u32);
					concatenated_product += product;
					last_product = product;
				}
				println!("Product found with {},{} = {}",i,j,concatenated_product);
				if concatenated_product >= answer{
					answer = concatenated_product;
				}
			}
		}
	}
	println!("The answer is {}",answer);
}
