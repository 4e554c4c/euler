use std::ops::Mul;
use std::ops::MulAssign;

struct Fraction {
	num: usize,
	den: usize,
}

impl Fraction {
	fn new(num: usize, den: usize) -> Fraction {
		Fraction {
			num: num,
			den: den,
		}
	}

	fn reduce(&self) -> Fraction {
		let gcd = {
			let mut a = self.num;
			let mut b = self.den;
			while b != 0 {
				let tmp = b;
				b = a % b;
				a = tmp;
			}
			a
		};
		Fraction {
			num: self.num / gcd,
			den: self.den / gcd,
		}
	}
}
impl Mul for Fraction {
	type Output = Fraction;

	fn mul(self, _rhs: Fraction) -> Fraction {
		Fraction {
			num: self.num * _rhs.num,
			den: self.den * _rhs.den
		}
	}
}
impl MulAssign for Fraction {
	fn mul_assign(&mut self, _rhs: Fraction) {
		self.num *= _rhs.num;
		self.den *= _rhs.den;
	}
}

impl PartialEq for Fraction {
	fn eq(&self, _rhs: &Fraction) -> bool {
		self.reduce().num == _rhs.reduce().num &&
		self.reduce().den == _rhs.reduce().den
	}
}

fn main() {
	let mut answer = Fraction::new(1,1);
	for i in 1..100 {
		for j in 1..i {
			let fract = Fraction::new(j,i);

			if fract.num % 10 == fract.den / 10 
			&&(fract.num % 10 != 0 && fract.den / 10 != 0) {
				let tmp = 
					Fraction::new(fract.num/10,fract.den%10);
				
				if tmp.num != 0 && tmp.den != 0
				&& tmp == fract {
					answer *= fract;
				}
			}
		}
	}
	println!("The answer is: {}/{}",answer.reduce().num,answer.reduce().den);
}
