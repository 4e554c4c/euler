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

