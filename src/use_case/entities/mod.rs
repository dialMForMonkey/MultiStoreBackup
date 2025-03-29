use bytes::Bytes;

pub(super) mod encrypt_symmetric_algorithm;
pub(super) mod encrypt_asymmetric_algorithm;



pub(super) struct EncryptedAsymmetricAlgorithm {
    pub public_key: Option<Vec<u8>>,
    pub private_key: Vec<u8>,
    pub encrypted_data: Vec<u8>
}

pub(super) struct EncryptedSymmetricalAlgorithm {
    pub encrypted_data: Vec<u8>,
    pub secret: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
}


pub(super) trait CryptographySymmetrical {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedSymmetricalAlgorithm, anyhow::Error>;
}

pub(super) trait CryptographyAsymmetrical {
    fn encrypt(data:Vec<u8>) -> Result<EncryptedAsymmetricAlgorithm, anyhow::Error>;
}