use crate::use_case::entities::{CryptographyAsymmetrical,  EncryptedAsymmetricAlgorithm};
use anyhow::{anyhow, Error};

use rsa::{RsaPrivateKey,RsaPublicKey,Pkcs1v15Encrypt};
use rsa::pkcs1::EncodeRsaPrivateKey;

struct RSA;
impl CryptographyAsymmetrical for RSA {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedAsymmetricAlgorithm, Error> {
        let bits = 4096;
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, bits)
            .expect("Error when create Private Key");

        let public_key =  RsaPublicKey::from(&private_key);

        let encrypted_result = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data);

        let private_key_der = private_key.to_pkcs1_der()?;


        match encrypted_result {
            Ok(result)=>{
                Ok(EncryptedAsymmetricAlgorithm {
                    public_key: None,
                    private_key: Vec::from(private_key_der.as_bytes()),
                    encrypted_data: result,
                })
            },
            Err(err) => {
                Err(anyhow!("Error when encrypt with RSA {}", err))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use rsa::pkcs1::der::Decode;
    use rsa::pkcs1::{DecodeRsaPrivateKey, RsaPrivateKey};
    use rsa::Pkcs1v15Encrypt;
    use crate::use_case::entities::CryptographyAsymmetrical;
    use crate::use_case::entities::encrypt_asymmetric_algorithm::RSA;

    #[test]
    fn rsa_test(){

        let data_to_encrypt = Vec::from("A sabedoria humana está nessas palavras: esperar e ter esperança.");

        let encrypted_result = RSA::encrypt(data_to_encrypt.clone());
        assert!(encrypted_result.is_ok());

        let encrypted_model = encrypted_result.unwrap();
        let private_key = rsa::RsaPrivateKey::from_pkcs1_der(encrypted_model.private_key.as_slice()).unwrap();

        let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, encrypted_model.encrypted_data.as_slice()).unwrap();
        assert_eq!(decrypted_data, data_to_encrypt)
    }
}







































