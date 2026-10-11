//! Generates a random BIP39 mnemonic (128 bits of entropy, 12 words)
//! and prints it as a single hyphen-separated passphrase.

use std::io::{self, Write};
use std::process::ExitCode;

use bip39::Mnemonic;
use zeroize::Zeroizing;

/// 16 bytes = 128 bits of entropy, which BIP39 encodes as 12 words.
const ENTROPY_BYTES: usize = 16;

fn generate() -> Result<Zeroizing<String>, Box<dyn std::error::Error>> {
    let mut entropy = Zeroizing::new([0u8; ENTROPY_BYTES]);
    getrandom::fill(entropy.as_mut())?;

    // Mnemonic zeroizes its word indices on drop.
    let mnemonic = Mnemonic::from_entropy(entropy.as_ref())?;

    let mut passphrase = Zeroizing::new(String::with_capacity(128));
    for (i, word) in mnemonic.words().enumerate() {
        if i > 0 {
            passphrase.push('-');
        }
        passphrase.push_str(word);
    }
    Ok(passphrase)
}

fn main() -> ExitCode {
    let passphrase = match generate() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("crypto-password: {e}");
            return ExitCode::FAILURE;
        }
    };

    let mut out = io::stdout().lock();
    if let Err(e) = writeln!(out, "{}", passphrase.as_str()).and_then(|()| out.flush()) {
        eprintln!("crypto-password: failed to write output: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_twelve_valid_words() {
        let p = generate().unwrap();
        let words: Vec<&str> = p.split('-').collect();
        assert_eq!(words.len(), 12);
        // Round-trips through the BIP39 checksum.
        Mnemonic::parse_normalized(&words.join(" ")).unwrap();
    }

    #[test]
    fn outputs_differ() {
        assert_ne!(*generate().unwrap(), *generate().unwrap());
    }
}
