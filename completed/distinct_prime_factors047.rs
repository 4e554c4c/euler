use std::vec;

fn prime_factors(n : u64) -> Vec<(u64,u8)>
{
	let mut num = n;
	let mut factors = Vec::new();
	while num != 1 {
		if num % 2 == 0{
			factors.push(2);
			num /= 2;
			continue;
		}
		if num % 3 == 0 {
			factors.push(3);
			num /= 3;
			continue;
		}
		for i in (1..).map(|x| x * 6) {
			if num % (i - 1) == 0 {
				factors.push(i-1);
				num /= i-1;
				break;
			}
			else if num % (i + 1) == 0 {
				factors.push(i+1);
				num /= i+1;
				break;
			}
			else if i >= num ^ 1/2 {
				factors.push(num);
				num = 1;
				break;
			}
		}
	}

	let mut ans= Vec::new();
	let mut times_used = 0;
	let mut last_value = factors[0];
	for i in factors {
		if i == last_value {
			times_used += 1;
			continue;
		}
		ans.push((last_value as u64,times_used));
		last_value = i;
		times_used = 1;
	}
	ans.push((last_value as u64,times_used));
	ans
}

fn main()
{
	let mut answer = 0;
	'counter: for i in 2.. {
		let mut facts = Vec::new();
		for j in i.. {
			if j-i > 3 {
				answer = i;
				break 'counter;
			}
			let new_facts = prime_factors(j);
			if new_facts.len() != 4 {
				break;
			}
			for k in new_facts {
				if !facts.contains(&k) {
					break;
				}
				facts.push(k);
			}
		}
	}
	println!("The answer is: {}",answer);
}
