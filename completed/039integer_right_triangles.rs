fn main()
{
	let mut answer = 0;
	let mut max = 0;
	for i in 3..1000 +1 {
		println!("Trying {}",i);
		let mut solutions = 0;
		for j in 0 as u64..i{
			for k in j..i{
				for l in k..i {
					if j.pow(2) + k.pow(2) ==l.pow(2) && j + k + l == i {
						solutions += 1;
					}
				}
			}
		}
		if solutions >= max {
			println!("Max found! {}",solutions);
			max = solutions;
			answer = i;
		}
	}
	println!("The answer is {}",answer);
}
