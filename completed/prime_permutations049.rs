extern crate bit_vec;
use bit_vec::BitVec;

fn is_permutation(a: usize, b: usize) -> bool
{
    let mut a = a;
    let mut b = b;
    let mut a_digits = Vec::new();
    let mut b_digits = Vec::new();
    while a != 0 {
        a_digits.push(a%10);
        a /= 10;
    }
    while b != 0 {
        b_digits.push(b%10);
        b /= 10;
    }
    a_digits.sort();
    b_digits.sort();
    a_digits == b_digits
}

const MIN: usize = 1000;
const MAX: usize = 10000;

fn main()
{
    let primes = {
        let mut bv = BitVec::from_elem(MAX,true);
        bv.set(0,false);bv.set(1,false);

        for i in (2 as usize..).take_while(|x| x.pow(2) <= MAX) {
            if bv[i]{
                for j in (i..).map(|x| x * i)
                              .take_while(|x| *x < MAX){
                    bv.set(j,false); }
            }
        }
        bv
    };

    for i in (MIN..MAX).filter(|&x| primes[x]) {
        for j in (1..MAX).take_while(|&x| i + 2 * x < MAX)
        .filter(|&x| primes[i + x] && primes[i + 2 * x]) {
            if is_permutation(i, i + j) && is_permutation(i, i + 2 * j){
                println!("{}{}{}",i,i+j,i+j+j);
            }
        }
    }
}
