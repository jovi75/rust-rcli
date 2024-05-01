use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{get_reader, process_genpass, TextSignFormat};

pub trait TextSign {
    /// Sign the data with the given key.
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify the data with the given key.
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    /// Load the key from the given path.
    fn load_key(path: impl AsRef<Path>) -> Self
    where
        Self: Sized;
}

pub trait KeyGenerator {
    /// Generate a new key.
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}
impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut r = OsRng;
        let sk = SigningKey::generate(&mut r);
        let vk = sk.verifying_key();
        Ok(vec![sk.to_bytes().to_vec(), vk.to_bytes().to_vec()])
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sign.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Blake3 {
    fn load_key(path: impl AsRef<Path>) -> Self {
        let key = fs::read(path).unwrap();
        Self::try_new(&key).unwrap()
    }
}

impl KeyLoader for Ed25519Signer {
    fn load_key(path: impl AsRef<Path>) -> Self {
        let key = fs::read(path).unwrap();
        Self::try_new(&key).unwrap()
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load_key(path: impl AsRef<Path>) -> Self {
        let key = fs::read(path).unwrap();
        Self::try_new(&key).unwrap()
    }
}

pub fn process_gen_key(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    let keys = match format {
        TextSignFormat::Blake3 => Blake3::generate()?,
        TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
    };
    Ok(keys)
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;

    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load_key(key);
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load_key(key);
            signer.sign(&mut reader)?
        }
    };
    let signed = BASE64_URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let signature = URL_SAFE_NO_PAD.decode(sig)?;
    let result = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load_key(key);
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load_key(key);
            verifier.verify(&mut reader, &signature)?
        }
    };
    Ok(result)
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let black3 = Blake3::load_key("fixture/blake3.txt");

        let data = b"hello world";
        let sig = black3.sign(&mut &data[..]).unwrap();
        assert!(black3.verify(&mut &data[..], &sig)?);

        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load_key("fixture/ed25519.sk");
        let pk = Ed25519Verifier::load_key("fixture/ed25519.pk");

        let data = b"hello world";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig)?);

        Ok(())
    }
}
