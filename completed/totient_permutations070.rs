extern crate bit_vec;
#[macro_use]
extern crate lazy_static;
use bit_vec::BitVec;
use std::vec::Vec;

static MAX: usize = 10_000_000;

lazy_static! {
    static ref SIEVE: BitVec = {
        let mut bv = BitVec::from_elem(MAX,true);
        bv.set(0,false);bv.set(1,false);

        for i in (2 as usize..).take_while(|x| x.pow(2) <= MAX) {
            if bv[i]{
                for j in (i..).map(|x| x * i)
                    .take_while(|&x| x < MAX){
                    bv.set(j,false);
                }
            }
        }
        bv
    };
}

fn gcd(u: usize, v: usize) -> usize {
    match (u,v) {
        (u,v) if u==v => u,
        (0,v) => v,
        (u,0) => u,
        // if u even
        (u,v) if (u & 1)==0 => {
            match v {
                // and v even
                v if (v & 1)==0 => {
                    gcd(u/2, v/2)*2
                },
                // or v odd
                v => gcd(u/2, v),
            }
        },
        // u odd, v even
        (u,v) if (v & 1)==0 => gcd(u,v/2),
        // reduce larger argument
        (u,v) if u > v => gcd((u-v)/2,v),
        (u,v) => gcd((v-u)/2,u),
    }
}

fn totient(n: usize) -> usize {
    match n {
        // by def
        0 => 0,
        1 => 1,
        // n prime => Lehmer's conjecture
        // true for all x < 10^20
        n if SIEVE[n] => n-1,
        // if even
        n if (n & 1) == 0 => {
            let m = n/2;
            if (m & 1) == 0 {
                2 * totient(m)
            } else {
                totient(m)
            }
        },
        // N is not prime, therefore there must exist some prime factor of n.
        // We will find this and recurse
        n => {
            // for all prime factors of n...
            for m in (2..MAX).filter(|&m| SIEVE[m] && n % m == 0) {
                let o = n / m;
                let d = gcd(m, o);
                return if d == 1 {
                    totient(m) * totient(o)
                } else {
                    totient(m) * totient(o) * d / totient(d)
                }
            }
            unreachable!();
        },
    }
}

fn is_permutation(x: (usize, usize)) -> bool
{
    let mut a = x.0;
    let mut b = x.1;
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
fn main() {
    let mut min = std::f64::INFINITY;
    let mut answer = 1;
    for n in 2..10_000_000 {
        let t = totient(n);

        if !is_permutation((n,t)) {
            continue;
        }

        let n_div_p = n as f64 / t as f64;
        if min != min.min(n_div_p) {
            min = n_div_p;
            answer = n;
        }
    }
    println!("The answer is: {}", answer);
}
