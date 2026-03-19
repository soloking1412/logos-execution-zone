use borsh::{BorshDeserialize, BorshSerialize};
use k256::ecdsa::signature::RandomizedSigner;
//use k256::schnorr::signature::Verifier;
pub use private_key::PrivateKey;
pub use public_key::PublicKey;
use rand::{RngCore as _, rngs::OsRng};

mod private_key;
mod public_key;

#[derive(Clone, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
pub struct Signature {
    pub value: [u8; 64],
}

impl std::fmt::Debug for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.value))
    }
}

impl Signature {
    #[must_use]
    pub fn new(key: &PrivateKey, message: &[u8]) -> Self {
        let mut aux_random = [0_u8; 32];
        OsRng.fill_bytes(&mut aux_random);
        Self::new_with_aux_random(key, message, aux_random)
    }

pub(crate) fn new_with_aux_random(
    key: &PrivateKey,
    message: &[u8],
    mut aux_random: [u8; 32],
) -> Self {
    let value = {
        // Create signing key from raw bytes
        let signing_key = k256::schnorr::SigningKey::from_bytes(key.value()).unwrap();

        // k256 expects a 32-byte message digest for Schnorr (BIP-340)
        let msg: &[u8; 32] = message.try_into().expect("message must be 32 bytes");

        // Convert aux_random into the expected type
        let aux: k256::elliptic_curve::FieldBytes = aux_random.into();

        // Sign with auxiliary randomness
        let signature: k256::schnorr::Signature = signing_key.sign_with_aux_rng(&aux, msg);

        signature.to_bytes()
    };

    Self { value }
}

    /*
    pub(crate) fn new_with_aux_random(
        key: &PrivateKey,
        message: &[u8],
        aux_random: [u8; 32],
    ) -> Self {
        let value = {
            let secp = secp256k1::Secp256k1::new();
            let secret_key = secp256k1::SecretKey::from_byte_array(*key.value()).unwrap();
            let keypair = secp256k1::Keypair::from_secret_key(&secp, &secret_key);
            let signature = secp.sign_schnorr_with_aux_rand(message, &keypair, &aux_random);
            signature.to_byte_array()
        };
        Self { value }
    }
*/
    #[must_use]
    pub fn is_valid_for(&self, bytes: &[u8], public_key: &PublicKey) -> bool /*{
    // Convert signature bytes into Signature object
    let sig_slice: &[u8] = &self.value; 
    let sig = match k256::schnorr::Signature::try_from(sig_slice) {
        Ok(s) => s,
        Err(_) => {panic!("TEST"); //return false
        },
    };

    // Convert x-only public key to VerifyingKey
    let vk = match k256::schnorr::VerifyingKey::from_bytes(public_key.value()) {
        Ok(vk) => vk,
        Err(_) => {panic!("TEST"); //return false
        },
    };

    // Verify the signature
  //  vk.verify(bytes, &sig).is_ok()
    
       //    let msg = hex32(&v.message);
      //  let sig_bytes = hex64(&v.signature);
    //    let sig = Signature::try_from(&sig_bytes[..]).unwrap();

  //      let vk_bytes = hex32(&v.public_key);
//        let vk = VerifyingKey::from_bytes(&vk_bytes).unwrap();

        // --- VERIFY ---
     //   let verify_ok = vk.verify_prehash(&msg, &sig).is_ok();
     

    }*/{
    
        let pk = secp256k1::XOnlyPublicKey::from_byte_array(*public_key.value()).unwrap();
        let secp = secp256k1::Secp256k1::new();
        let sig = secp256k1::schnorr::Signature::from_byte_array(self.value);
        secp.verify_schnorr(&sig, bytes, &pk).is_ok()
    }
    
    
}

#[cfg(test)]
mod bip340_test_vectors;

#[cfg(test)]
mod tests {

    use crate::{Signature, signature::bip340_test_vectors};

    impl Signature {
        pub(crate) fn new_for_tests(value: [u8; 64]) -> Self {
            Self { value }
        }
    }

    #[test]
    fn signature_generation_from_bip340_test_vectors() {
        for (i, test_vector) in bip340_test_vectors::test_vectors().into_iter().enumerate() {
            let Some(private_key) = test_vector.seckey else {
                continue;
            };
            let Some(aux_random) = test_vector.aux_rand else {
                continue;
            };
            let Some(message) = test_vector.message else {
                continue;
            };
            if !test_vector.verification_result {
                continue;
            }
            let expected_signature = &test_vector.signature;

            let signature = Signature::new_with_aux_random(&private_key, &message, aux_random);

            assert_eq!(&signature, expected_signature, "Failed test vector {i}");
        }
    }

    #[test]
    fn signature_verification_from_bip340_test_vectors() {
        for (i, test_vector) in bip340_test_vectors::test_vectors().into_iter().enumerate() {
            let message = test_vector.message.unwrap_or(vec![]);
            let expected_result = test_vector.verification_result;

            let result = test_vector
                .signature
                .is_valid_for(&message, &test_vector.pubkey);

            assert_eq!(result, expected_result, "Failed test vector {i}");
        }
    }
}
