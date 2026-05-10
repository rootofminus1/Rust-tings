



pub trait KeyAgreement {
    type PrivateKey;
    type PublicKey;
    type SharedSecret;

    fn generate_secret_key(&self) -> Self::PrivateKey;
    fn derive_public_key(&self, sk: &Self::PrivateKey) -> Self::PublicKey;
    fn agree(&self, sk: &Self::PrivateKey, their_pk: &Self::PublicKey) -> Self::SharedSecret;

    // has a default impl as it just generates and derives
    fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey) {
        let sk = self.generate_secret_key();
        let pk = self.derive_public_key(&sk);
        (sk, pk)
    }

}

pub trait Kem {
    type PrivateKey;
    type PublicKey;
    type Ciphertext;
    type SharedSecret;

    
    fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey);

    fn encapsulate(&self, pk: &Self::PublicKey) -> (Self::SharedSecret, Self::Ciphertext);
    fn encapsulate_with_esk(&self, pk: &Self::PublicKey, esk: &Self::PrivateKey) -> (Self::SharedSecret, Self::Ciphertext);

    fn decapsulate(&self, sk: &Self::PrivateKey, ct: &Self::Ciphertext) -> Self::SharedSecret;
}

pub struct KemFromKa<KA: KeyAgreement> {
    ka: KA,
}

impl<KA: KeyAgreement> KemFromKa<KA> {
    pub fn new(ka: KA) -> Self {
        Self { ka }
    }
}

impl<KA: KeyAgreement> Kem for KemFromKa<KA> {
    type PrivateKey = KA::PrivateKey;
    type PublicKey = KA::PublicKey;
    type Ciphertext = KA::PublicKey;  // the ephemral public key is the ciphertext
    type SharedSecret = KA::SharedSecret;

    fn generate_keypair(&self) -> (Self::PrivateKey, Self::PublicKey) {
        self.ka.generate_keypair()
    }

    fn encapsulate(&self, pk: &Self::PublicKey) -> (Self::SharedSecret, Self::Ciphertext) {
        let esk= self.ka.generate_secret_key();
        self.encapsulate_with_esk(pk, &esk)
    }

    fn encapsulate_with_esk(&self, pk: &Self::PublicKey, esk: &Self::PrivateKey) -> (Self::SharedSecret, Self::Ciphertext) {
        let epk = self.ka.derive_public_key(esk);
        let shared = self.ka.agree(esk, pk);
        (shared, epk)
    }

    fn decapsulate(&self, sk: &Self::PrivateKey, ct: &Self::Ciphertext) -> Self::SharedSecret {
        self.ka.agree(sk, ct)
    }
}
