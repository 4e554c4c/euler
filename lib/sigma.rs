// self explanitory. Unit is not prime
fn prime_factors(n : u64) -> Vec<u64>
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
	factors
}

// [sigma](0,num) finds the number of divisors of num
fn sigma_0(num : u64) -> u64
{
	let factors = prime_factors(num);
	let mut exponents = Vec::new();

	{
		let mut times_used = 0;
		let mut last_value = factors[0];
		for i in 0..factors.len() {
			times_used += 1;
			if factors[i] != last_value || i == factors.len() - 1 {
				exponents.push(times_used);
			}
			last_value = factors[i];
		}
	}
	let mut product = 1;
	for i in exponents {
		product *= i + 1;
	}
	product
}
// [sigma](1,num) finds the sum of all divisors of num
fn sigma_1(num : u64) -> u64
{
	let mut unique_factors = Vec::new();
	let mut exponents = Vec::new();

	{
		let factors = prime_factors(num);
		let mut times_used = 0;
		let mut last_value: u64;
		match factors.first() {
			Some(x) => last_value = *x,
			None	=> 
				panic!("Err: no factors returned from prime_factors"),
		}
		unique_factors.push(last_value);
		for i in factors {
			if i == last_value {
				times_used += 1;
				continue;
			}
			last_value = i;
			exponents.push(times_used);
			unique_factors.push(i);
			times_used = 1;
		}
		exponents.push(times_used);
		assert_eq!(unique_factors.len(),exponents.len());
	}
	let mut product = 1; 

	for i in 0..unique_factors.len(){
		product *= 
			(unique_factors[i].pow(exponents[i] + 1) - 1) /
			(unique_factors[i] - 1);
	}
	
	product
}
