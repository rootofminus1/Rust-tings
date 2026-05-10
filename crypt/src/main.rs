use crate::{dhecdh::{DhKa, EcdhKa}, elliptic_curve::{EllipticCurve, Point}, kakem::{Kem, KemFromKa, KeyAgreement}};

mod operations;
mod elliptic_curve;
mod kakem;
mod dhecdh;


pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;


fn main() -> Result<(), Error> {

    dhka();
    dhkem();
    ecdhka();
    ecdhkem();

    Ok(())
}

fn dhka() {
    let dhka = DhKa::new(23, 5);

    let (ask, apk) = dhka.generate_keypair();
    let (bsk, bpk) = dhka.generate_keypair();

    let ass = dhka.agree(&ask, &bpk);
    let bss = dhka.agree(&bsk, &apk);

    println!("DH Key Agreement");
    println!("a: {}", ass);
    println!("b: {}", bss);
}

fn dhkem() {
    // let dhkem = DhKem::new(23, 5);
    let dhkem = KemFromKa::new(DhKa::new(23, 5));

    // bob's sk/pk
    let (sk, pk) = dhkem.generate_keypair();
    // alice encapsulates and can share ct with bob
    let (ass, ct) = dhkem.encapsulate(&pk);
    // bob can decapsulate with the ct
    let bss = dhkem.decapsulate(&sk, &ct);

    println!("DH KEM)");
    println!("a: {}", ass);
    println!("b: {}", bss);
}




fn ecdhka() {
    // example
    // E: y^2 = x^3 - 3x + 1
    // over F_13
    // P = (0, 1)
    // ord(P) = 19

    let n = 13;
    let e = EllipticCurve::new(-3, 1, n);
    let point = Point::Affine(0, 1);
    let order = 19;

    let ecdhka = EcdhKa::new(e,  point, order);

    let (ask, apk) = ecdhka.generate_keypair();
    let (bsk, bpk) = ecdhka.generate_keypair();

    let ass = ecdhka.agree(&ask, &bpk);
    let bss = ecdhka.agree(&bsk, &apk);

    println!("ECDH Key Agreement");
    println!("a: {:?}", ass);
    println!("b: {:?}", bss);
}

fn ecdhkem() {
    let n = 13;
    let curve = EllipticCurve::new(-3, 1, n);
    let p = Point::Affine(0, 1);
    let order = 19;

    let ecdh_kem  = KemFromKa::new(EcdhKa::new(curve,  p, order));

    // bob's sk/pk
    let (sk, pk) = ecdh_kem.generate_keypair();
    // alice encapsulates and can share ct with bob
    let (ass, ct) = ecdh_kem.encapsulate(&pk);
    // bob can decapsulate with the ct
    let bss = ecdh_kem.decapsulate(&sk, &ct);

    println!("ECDH KEM)");
    println!("a: {:?}", ass);
    println!("b: {:?}", bss);
}


// unused, mental model example
fn dh() {
    let p = 23;  
    let g: i32 = 5;

    let a_s = 4;
    let a_p = g.pow(a_s) % p;

    let b_s = 3;
    let b_p = g.pow(b_s) % p;

    let s_a = b_p.pow(a_s) % p;
    let s_b = a_p.pow(b_s) % p;


    println!("DH\nsa: {} sb: {}", s_a, s_b);
}




