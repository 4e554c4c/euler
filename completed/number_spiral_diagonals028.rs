fn main()
{
	println!("The answer is: {}",(1 as i64..)
		.take((5 as usize).pow(2))
		.map(|x| (x,(x as f64).sqrt().ceil() as i64 + (((x as f64).sqrt().ceil() as i64%2 == 0) as i64)))
		.filter(|x| x.0 == (x.1 - 2).pow(2) + (x.1-1)
				 || x.0 == (x.1 - 2).pow(2) + 2*(x.1-1)
				 || x.0 == (x.1).pow(2) - (x.1 - 1)
				 || x.0 == (x.1).pow(2))
		.fold(0,|acc, x| acc + x.0));
}
