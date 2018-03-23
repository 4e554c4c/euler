use bit_vec::BitVec;
use std::vec::Vec;

extern crate bit_vec;

const MAX: usize = 10_000_000;

fn to_vec(num: usize, radix: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    let mut num = num;

    while num != 0   {
        vec.push(num % radix);
        num /= radix;
    }
    vec
}

fn main() {
    // Primes to check
    let primes = {
        let mut bv = BitVec::from_elem(MAX,true);
        bv.set(0,false);bv.set(1,false);

        for i in (2 as usize..).take_while(|x| x.pow(2) <= MAX) {
            if bv[i]{
                for j in (i..).map(|x| x * i)
                              .take_while(|&x| x < MAX){
                    bv.set(j,false); }
            }
        }
        bv
    };
    let mut unchecked_primes = primes.clone();
    let mut max_count = 0;
    let mut answer = 0;
    // Quickly iterate over primes of the form Z*6+-1 (all primes except 2,3)
    for p in (1usize..).flat_map(|x|
            std::iter::once(x*6 - 1).chain(
                std::iter::once(x*6 + 1))).take_while(|&x| x <= MAX) {
        // If it's not a prime or we've seen it, continue.
        if !unchecked_primes[p] {
            continue;
        }
        let digits = to_vec(p, 10);
        let len = digits.len() as u32;
        // Now choose k digits out of n-1
        for k in 1u32..len {
            // All different combinations of k digits represented as a binary
            // number. 1s are the "chosen", zeros are "unchosen"
            for c in (1usize..).filter(|&x| k == x.count_ones())
                    .take_while(|&x| x < 2usize.pow(len-1))
                    .map(|x| x << 1) {
                let mut count = 0;
                let mut smallest = p;
                // try each digit replacement
                for i in (0..10).rev() {
                    if i == 0 && c & 2usize.pow(len-1) != 0 {
                        // We can't replace the first digit if it's zero
                        continue;
                    }
                    // Turn it back into a number with the digits replaced
                    let num = digits.iter().enumerate().map(|(d,&x)| {
                        if c & 1 << d != 0 {
                            i
                        } else {
                            x
                        }
                    }).rev().fold(0, |acc, x| (acc * 10) + x);
                    if primes[num] {
                        count += 1;
                        smallest = num;
                        unchecked_primes.set(num, false);
                    }
                }
                if count > max_count {
                    max_count = count;
                    answer = smallest;
                }
            }
        }
    }
    println!("The answer is: {} with count {}" , answer, max_count);
}
