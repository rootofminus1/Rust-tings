use roots::{FloatType, Roots};


pub fn modn(x: i32, n: u64) -> i32 {
    ((x % n as i32) + n as i32) % n as i32
}

pub fn mod_inv(x: i32, p: u64) -> i32 {
    let x = modn(x, p);

    if x == 0 {
        panic!("0 has no inverse");
    }

    for i in 1..p {
        if (x * i as i32) % p as i32 == 1 {
            return i as i32;
        }
    }

    panic!("no inverse");
}


pub fn modpow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1; // a^0 = 1
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }

        base = (base * base) % modulus;
        exp >>= 1; 
    } 

    result
}

pub fn modpow2(a: u64, n: u64, m: u64) -> u64 {
    match n {
        0 => 1,
        n if n % 2 == 0 => { 
            modpow2((a * a) % m, n / 2, m)
        },
        _  => { 
            (a * modpow((a * a) % m, (n - 1) / 2, m)) % m
        }
    }
}

pub fn roots_to_vec<T: FloatType>(roots: Roots<T>) -> Vec<T> {
    match roots {
        Roots::No(_) => vec![],
        Roots::One(r) => r.to_vec(),
        Roots::Two(r) => r.to_vec(),
        Roots::Three(r) => r.to_vec(),
        Roots::Four(r) => r.to_vec(),
    }
}
