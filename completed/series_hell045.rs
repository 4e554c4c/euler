fn is_perfect_square(n: u64) -> bool
{
	match n & 0xF{
		0x0 | 0x1 | 0x4 | 0x9 =>
			((n as f64).sqrt().floor() as u64)
			.pow(2) == n,
		_ => false
	}
}
fn is_triangular(n: u64) -> bool
{
	let n = 8*n + 1;
	is_perfect_square(n)
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
fn is_hexagonal(n: u64) -> bool
{
	let n = 8*n + 1;
	is_perfect_square(n) && (n+1) % 2 == 0
}
fn main()
{
	for i in (1..)
		.filter(|&x| is_triangular(x))
		.filter(|&x| is_pentagonal(x))
		.filter(|&x| is_hexagonal(x)){
		println!("Found answer {}",i);
	}
}
