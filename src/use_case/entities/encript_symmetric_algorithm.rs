use aes_gcm::aead::OsRng;
use aes_gcm::{AeadCore, AeadInPlace, Aes256Gcm, Key as aes_gcm_key};
use anyhow::anyhow;
use chacha20::cipher::KeyInit;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use chacha20::{Key, XChaCha20, XNonce};

use crate::use_case::entities::{CryptographySymmetrical, EncryptedSymmetricalAlgorithm};

struct Aes;
struct ChaCha20;

impl CryptographySymmetrical for Aes {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedSymmetricalAlgorithm, anyhow::Error>{

        let key = Aes256Gcm::generate_key(OsRng);
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(OsRng);
        let mut encrypted_data: Vec<u8>  = data.clone();
        let result = cipher.encrypt_in_place(&nonce,b"", &mut encrypted_data);

        match result {
            Err(err) => {
                return Err(anyhow!("error encryption in aes_gcm {:?}",err));
            },Ok(x)=>{

            }
        }

        Ok(EncryptedSymmetricalAlgorithm {
            encrypted_data,
            secret: key.to_vec(),
            nonce: Some(nonce.to_vec()),
        })
    }
}

impl CryptographySymmetrical for ChaCha20 {

    fn encrypt(data:Vec<u8>) -> Result<EncryptedSymmetricalAlgorithm, anyhow::Error>{
        let key = Key::default();
        let nonce = XNonce::default();

        let mut block = XChaCha20::new(&key, &nonce.into());
        let mut encrypted_data: Vec<u8>  =  vec![0; data.len()];


        let result =  block.apply_keystream_b2b(data.as_slice(), &mut encrypted_data);

        match result {
            Err(err) => {
                return Err(anyhow!("error encryption in cha_cha_20 {:?}",err));
            },Ok(x)=>{

            }
        }




        Ok(EncryptedSymmetricalAlgorithm {
            encrypted_data,
            secret: key.to_vec(),
            nonce:  Some(nonce.to_vec()),
        })
    }
}





#[cfg(test)]
mod tests {

    use aes_gcm::{AeadInPlace, KeyInit};
    use aes_gcm::Aes256Gcm;
    use chacha20::cipher::{KeyIvInit, StreamCipher};
    use chacha20::{Key, XNonce};
    use crate::use_case::entities::CryptographySymmetrical;

    #[test]
    fn cha_cha_20_encrypt_test() {
        let data_to_encrypt = "A sabedoria humana está nessas palavras: esperar e ter esperança.";


        let encrypted = super::ChaCha20::encrypt(Vec::from(data_to_encrypt));
        assert!(encrypted.is_ok());
        let mut encrypted_model = encrypted.unwrap();

        assert_ne!(encrypted_model.encrypted_data.len(), 0);


        let key = Key::from_slice(encrypted_model.secret.as_slice());
        let nonce = encrypted_model.nonce.unwrap();
        let nonce = XNonce::from_slice(nonce.as_slice());

        let mut cha = chacha20::XChaCha20::new(key, nonce);
        cha.apply_keystream(encrypted_model.encrypted_data.as_mut_slice());
        assert_eq!(encrypted_model.encrypted_data.as_mut_slice(), Vec::from(data_to_encrypt));
    }


    #[test]
    fn aes_encrypt_test() {
        let data_to_encrypt = "A sabedoria humana está nessas palavras: esperar e ter esperança.";
        let data_to_encrypt = Vec::from(data_to_encrypt);

        let encrypted = super::Aes::encrypt(data_to_encrypt.clone());
        assert!(encrypted.is_ok());

        let encrypted_model = encrypted.unwrap();

        let cipher = Aes256Gcm::new_from_slice(encrypted_model.secret.as_slice());
        let cipher  = cipher.unwrap();
        let nonce = encrypted_model.nonce.unwrap();
        let mut decrypted_data: Vec<u8>  = encrypted_model.encrypted_data.clone();
        let _ = cipher.decrypt_in_place(aes_gcm::Nonce::from_slice(nonce.as_slice()), b"",&mut decrypted_data);

        assert_eq!(data_to_encrypt,  decrypted_data);


    }
}
