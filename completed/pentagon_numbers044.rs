//I hate myself so I inlined this
fn is_perfect_square(n: u64) -> bool
{
	match n & 0xF {
		0x0 | 0x1 | 0x4 | 0x9
			=> n == ((n as f64).sqrt().floor() as u64).pow(2),
		_
			=> false
	}
}

fn is_pentagonal(n: u64) -> bool
{
	let m = 24*n + 1;
	match m & 0xF {
		0x0 | 0x1 | 0x4 | 0x9 =>
			match (m as f64)
				.sqrt().floor() as u64 {
				x if x.pow(2) == m
					=> (x+1) % 6 == 0,
				_   => false
			},
		_   => false
	}
}

fn main()
{
	/*
	for i in (0..200).filter(|&x| is_pentagonal(x)) {
		println!("Found pentagonal number {}",i);
	}
	*/
	let mut D = 1000000;

	for i in (1..).map(|x| (x * (3*x - 1) / 2)){
		for j in (1 as u64..).map(|x| (x * (3*x - 1) / 2))
			.take_while(|&x| x < i){

			if i - j < D {
				continue;
			}
			if is_pentagonal(i + j) && is_pentagonal(i - j){
				D = i-j;
				println!("Found pair {} - {} = {}",i,j,D);
			}
		}
	}
	println!("The answer is {}",D);
}
