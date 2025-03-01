use bytes::Bytes;

mod encript_symmetric_algorithm;
mod encript_asymmetric_algorithm;



struct EncryptedAsymmetricAlgorithm {
    public_key: Option<Vec<u8>>,
    private_key: Vec<u8>,
    encrypted_data: Vec<u8>
}

pub struct EncryptedSymmetricalAlgorithm {
    encrypted_data: Vec<u8>,
    secret: Vec<u8>,
    nonce: Option<Vec<u8>>,
}


pub trait CryptographySymmetrical {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedSymmetricalAlgorithm, anyhow::Error>;
}

pub trait CryptographyAsymmetrical {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedAsymmetricAlgorithm, anyhow::Error>;
}