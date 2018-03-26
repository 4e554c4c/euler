use std::cmp::Ordering;

fn is_perm_of(nums: &Vec<u64>,set: &Vec<u64>) -> bool
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
	digits == *set
}
fn main()
{
	let mut answer = 0;
	let digits = vec![1,2,3,4,5,6,7,8,9];
	// n!.pow(2) too many :c
	// 1. Generate permutations
	//  . find all pairs of * and =
	//  . where 0 < *,=, < n
	//  . takes about O(n!.pow(2))
	//
	// 2. iterate between two numbers
	//  . a & b, where 1 < a,b < some num
	//  . some num <= 10,000??? for now i guess
	//  . find if a || b || a*b is a permutation of 1..digits+1
	for i in 2..10000 {
		for j in 2..10000 {
			if i == j { break; }
			if is_perm_of(&vec![i,j,i*j],&digits) {
				println!("Found Pandigital Product: {} * {} = {}"
					,i,j,i*j);
				answer += i*j;
			}
		}
	}
	println!("The answer is: {}",answer);
}
