fn reciprocal(n: u64) -> u64
{
	assert!(n != 0);

	let mut counter = 0;
	let mut div = Vec::new();
	div.push(10);
	for i in 1.. {
		match div.last().unwrap() {
			&0 => {
				break;
			}
			&d if d < n => {
				div.push(d * 10);
			}
			&d => {
				if div.contains(&((d%n) * 10)) {
					counter = i;
					break;
				}
				div.push((d%n) * 10);
			}
		}
	}
	return counter;
}

fn main()
{
	let answer = {
		let mut answer = 0;
		let mut max = 0;
		for i in 1..1000 {
			let tmp = reciprocal(i);
			if tmp > max {
				max = tmp;
				answer = i;
			}
		}
		answer
	};
	println!("The answer is: {}",answer);
}
