use std::io;

fn divides_from(upto: i64, num: i64) -> bool {
	for j in 1..upto {
		if num%j != 0{
			return false;
		}
	}
	true
}
fn main(){
	let upto = 20;
	let mut answer = 0;
	for i in upto..{
		if divides_from(upto, i) {
			println!("{}", i);
			break;
		}
	}
}
