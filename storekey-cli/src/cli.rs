use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use anyhow::Result;
use base64::prelude::*;
use clap::{Parser, Subcommand};
use sha3::{Digest, Sha3_256};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Encrypt {
        /// Seed Phrase
        #[arg(short)]
        seed_phrase: String,
        /// Master Password
        #[arg(short)]
        password: String,
    },
    Decrypt {
        /// Encrypted Data
        #[arg(short)]
        encrypted_string: String,
        /// Master Password
        #[arg(short)]
        password: String,
    },
}

pub enum CommandResult {
    EncryptResult(String),
    DecryptResult(String),
}

impl Commands {
    pub fn execute(self) -> Result<CommandResult> {
        match self {
            Self::Encrypt {
                seed_phrase,
                password,
            } => {
                let mut hasher = Sha3_256::new();

                hasher.update(password);

                let password_hash = hasher.finalize();

                let key = Key::<Aes256Gcm>::from_slice(password_hash.as_slice());

                let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

                let cipher = Aes256Gcm::new(key);

                let ciphered_data = cipher
                    .encrypt(&nonce, seed_phrase.as_bytes())
                    .expect("Failed to encrypt the data.");

                // Combining nonce and encrypted data together for storage purposes
                let mut encrypted_data: Vec<u8> = nonce.to_vec();

                encrypted_data.extend_from_slice(&ciphered_data);

                Ok(CommandResult::EncryptResult(
                    BASE64_STANDARD.encode(&encrypted_data),
                ))
            }
            Self::Decrypt {
                encrypted_string,
                password,
            } => {
                let mut hasher = Sha3_256::new();

                hasher.update(password);

                let password_hash = hasher.finalize();

                let key = Key::<Aes256Gcm>::from_slice(password_hash.as_slice());

                let encrypted_data = BASE64_STANDARD.decode(&encrypted_string)?;

                let (nonce_slice, ciphered_data) = encrypted_data.split_at(12);

                let nonce = Nonce::from_slice(nonce_slice);

                let cipher = Aes256Gcm::new(key);

                let seed_phrase = cipher
                    .decrypt(nonce, ciphered_data)
                    .expect("Failed to decrypt the data.");

                Ok(CommandResult::DecryptResult(String::from_utf8(
                    seed_phrase,
                )?))
            }
        }
    }
}
