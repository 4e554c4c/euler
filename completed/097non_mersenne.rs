fn mul_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut d = 0;
    a %= m;
    b %= m;
    for _ in 0..64 {
        d = if d > m/2 {
            (d << 1) - m
        } else {
            d << 1
        };
        if a & 1 << 63 != 0 {
            d += b;
        }
        if d >= m {
            d -= m;
        }
        a <<= 1;
    }
    d
}
fn exp_mod(mut a: u64, mut b: u64, m: u64) -> u64 {
    let mut r = 1;
    while b > 0 {
        if b % 2 == 1 {
            r = mul_mod(r, a, m);
        }
        b = b >> 1;
        a = mul_mod(a, a, m);
    }
    r
}

fn main() {
    println!("The answer is {}",
             mul_mod(28_433,exp_mod(2, 7830457, 10_000_000_000),
                     10_000_000_000)+1);
}
