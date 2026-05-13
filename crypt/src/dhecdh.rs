use rand::random_range;

use crate::{elliptic_curve::{EllipticCurve, Point}, kakem::KeyAgreement, operations::modpow};

// TODO: remove (unused anymore, can be derived from groups)

pub struct EcdhKa {
    curve: EllipticCurve,
    generator: Point,
    order: u64,  // the order of the generator, ord((0, 1)) = 19 on y^2 = x^3 - 3x + 1 in F_13
}

impl EcdhKa {
    // EC field modulo and n have to agree
    pub fn new(curve: EllipticCurve, generator: Point, order: u64) -> Self {
        Self { curve, generator, order, }
    }
}

impl KeyAgreement for EcdhKa {
    type PrivateKey = u64;

    type PublicKey = Point;

    type SharedSecret = Point;

    fn generate_secret_key(&self) -> Self::PrivateKey {
        random_range(2..self.order-1)
    }

    fn derive_public_key(&self, sk: &Self::PrivateKey) -> Self::PublicKey {
        self.curve.mult(*sk, self.generator)
    }

    // fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey) {
    //     let sk = random_range(2..self.n-1);
    //     let pk = self.derive_public_key(&sk);

    //     (sk, pk)
    // }

    fn agree(&self, sk: &Self::PrivateKey, their_pk: &Self::PublicKey) -> Self::SharedSecret {
        self.curve.mult(*sk, *their_pk)
    }


    

}



pub struct DhKa {
    p: u64,
    g: u64
}

impl DhKa {
    pub fn new(p: u64, g: u64) -> Self {
        Self { p, g }
    }
}

impl KeyAgreement for DhKa {
    type PrivateKey = u64;
    type PublicKey = u64;
    type SharedSecret = u64;


    fn generate_secret_key(&self) -> Self::PrivateKey {
        random_range(2..self.p-1)
    }

    // fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey) {
    //     let sk = random_range(2..self.p-1);
    //     let pk = self.derive_public_key(&sk);

    //     (sk, pk)
    // }

    fn agree(&self, sk: &Self::PrivateKey, their_pk: &Self::PublicKey) -> Self::SharedSecret {
        modpow(*their_pk, *sk, self.p)
    }

    fn derive_public_key(&self, sk: &Self::PrivateKey) -> Self::PublicKey {
        modpow(self.g, *sk, self.p)
    }
}




// struct DhKem {
//     p: u64,
//     g: u64
// }

// impl DhKem {
//     pub fn new(p: u64, g: u64) -> Self {
//         Self { p, g }
//     }
// }

// impl Kem for DhKem {
//     type PrivateKey = u64;
//     type PublicKey = u64;
//     type Ciphertext = u64;
//     type SharedSecret = u64;


//     // can be used or not, if not then picked (insecure)
//     fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey) {        
//         let sk = random_range(2..self.p-1);
//         let pk = modpow(self.g, sk, self.p);

//         (sk, pk)
//     }
    
//     fn encapsulate(&self, pk: &Self::PublicKey) -> (Self::SharedSecret, Self::Ciphertext) {
//         // my own pk/sk that i DO NOT get to decide on:
//         let esk = random_range(2..self.p-1);

//         self.encapsulate_with_esk(pk, &esk)
//     }

//     fn encapsulate_with_esk(&self, pk: &Self::PublicKey, esk: &Self::PrivateKey) -> (Self::SharedSecret, Self::Ciphertext) {
//         let epk = modpow(self.g, *esk, self.p);
//         let shared = modpow(*pk, *esk, self.p);
//         (shared, epk)
//     }
    
//     fn decapsulate(&self, sk: &Self::PrivateKey, ct: &Self::Ciphertext) -> Self::SharedSecret {
//         modpow(*ct, *sk, self.p)
//     }
// }



