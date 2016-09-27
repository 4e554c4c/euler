use std::io;

fn main(){
	let mut square_sum = 0;
	let mut sum_square = 0;
	for i in 1..101{
		square_sum += i*i;
	}
	for i in 1..101{
		sum_square += i;
	}
	sum_square *= sum_square;
	println!("{} - {} = {}",sum_square, square_sum, sum_square-square_sum);
}
