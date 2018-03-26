fn lattice_paths(c1 : u8, c2 : u8) -> u64 
{
	if c1 == 20 || c2 == 20{ return 1; }
	lattice_paths(c1 + 1, c2) + lattice_paths(c1, c2 + 1)
}

fn main()
{
	println!("The answer is {}",lattice_paths(0,0));
}
