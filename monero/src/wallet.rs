//! Monero Wallet generator

use network::{get_prefix, Network};

use base58::ToBase58;
use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, scalar::Scalar};
use hex_slice::HexSlice;
use rand::Rng;
use rand::rngs::OsRng;
use serde::Serialize;
use std::fmt;
use tiny_keccak::keccak256;

/// Represents Monero keypairs
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoneroWallet {
    /// public monero address
    pub address: String,
    /// private spend key
    #[serde(rename = "spend_key")]
    pub private_spend_key: [u8; 32],
    /// private view key
    #[serde(rename = "view_key")]
    pub private_view_key: [u8; 32],
    /// public spend key
    #[serde(skip_serializing)]
    pub public_spend_key: [u8; 32],
    /// public view key
    #[serde(skip_serializing)]
    pub public_view_key: [u8; 32],
}

impl MoneroWallet {
    /// Generates a new MoneroWallet for a given `network`
    pub fn new(network: &Network) -> MoneroWallet {
        let mut csprng: OsRng = OsRng::new().unwrap();
        Self::from_seed(network, csprng.gen())
    }

    /// Generates a MoneroWallet for a given `network` from a seed.
    pub fn from_seed(network: &Network, seed: [u8; 32]) -> MoneroWallet {
        let private_spend_key = Scalar::from_bytes_mod_order(seed).to_bytes();
        let hash = keccak256(&private_spend_key);
        let private_view_key = Scalar::from_bytes_mod_order(hash).to_bytes();
        let public_spend_key = MoneroWallet::scalar_mul_by_b_compressed(&private_spend_key);
        let public_view_key = MoneroWallet::scalar_mul_by_b_compressed(&hash);
        let address =
            MoneroWallet::generate_address(&network, &public_spend_key, &public_view_key);

        MoneroWallet {
            address,
            private_spend_key,
            private_view_key,
            public_spend_key,
            public_view_key,
        }
    }

    pub fn generate_address(
        network: &Network,
        spend_key: &[u8; 32],
        view_key: &[u8; 32],
    ) -> String {
        let mut bytes = Vec::new();

        // Add coin prefix
        match get_prefix(&network) {
            Some(prefix) => bytes.extend(prefix.iter().cloned()),
            None => panic!("Invalid prefix"), // make more descriptive
        };

        // Add public keys
        bytes.extend(spend_key.iter().cloned());
        bytes.extend(view_key.iter().cloned());

        // Add checksum
        let hash = &keccak256(bytes.as_slice())[..4];
        bytes.extend(hash.iter().cloned());

        // Convert to base58 in 8 byte chunks
        let mut base58 = String::new();
        for chunk in bytes.as_slice().chunks(8) {
            let mut part = chunk.to_base58();
            let exp_len = match chunk.len() {
                8 => 11,
                6 => 9,
                5 => 7,
                _ => panic!("Invalid chunk length: {}", chunk.len()),
            };
            let missing = exp_len - part.len();
            if missing > 0 {
                part.insert_str(0, &"11111111111"[..missing]);
            }
            base58.push_str(&part);
        }

        base58
    }

    /// returns public address
    pub fn address(&self) -> &String {
        &self.address
    }

    /// returns spend private key
    pub fn private_spend_key(&self) -> String {
        HexSlice::new(&self.private_spend_key).format()
    }

    /// returns spend private key as [u8]
    pub fn private_spend_key_u8(&self) -> &[u8; 32] {
        &self.private_spend_key
    }

    /// returns view private key
    pub fn private_view_key(&self) -> String {
        HexSlice::new(&self.private_view_key).format()
    }

    /// returns view private key as [u8]
    pub fn private_view_key_u8(&self) -> &[u8; 32] {
        &self.private_view_key
    }

    /// returns spend public key
    pub fn public_spend_key(&self) -> String {
        HexSlice::new(&self.public_spend_key).format()
    }

    /// returns spend public key as [u8]
    pub fn public_spend_key_u8(&self) -> &[u8; 32] {
        &self.public_spend_key
    }

    /// returns view public key
    pub fn public_view_key(&self) -> String {
        HexSlice::new(&self.public_view_key).format()
    }

    /// returns view public key as [u8]
    pub fn public_view_key_u8(&self) -> &[u8; 32] {
        &self.public_view_key
    }

    fn scalar_mul_by_b_compressed(bits: &[u8; 32]) -> [u8; 32] {
        let point = &Scalar::from_bits(*bits) * &ED25519_BASEPOINT_TABLE;
        let compressed = *point.compress().as_bytes();
        compressed
    }
}

impl fmt::Display for MoneroWallet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
        Address:              {}
        Private Spend Key:    {}
        Private View Key:     {}
        ",
            self.address(),
            self.private_spend_key(),
            self.private_view_key()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_new_wallet() {
        let network = Network::Mainnet;
        let wallet = MoneroWallet::new(&network);

        let private_spend_key = wallet.private_spend_key_u8();

        let hash = keccak256(private_spend_key);
        let expected_private_view_key = Scalar::from_bytes_mod_order(hash).to_bytes();
        let expected_public_spend_key = MoneroWallet::scalar_mul_by_b_compressed(&private_spend_key);
        let expected_public_view_key = MoneroWallet::scalar_mul_by_b_compressed(&hash);
        let expected_address =
            MoneroWallet::generate_address(&network, &expected_public_spend_key, &expected_public_view_key);
        
        assert_eq!(*wallet.private_view_key_u8(), expected_private_view_key);
        assert_eq!(*wallet.public_spend_key_u8(), expected_public_spend_key);
        assert_eq!(*wallet.public_view_key_u8(), expected_public_view_key);
        assert_eq!(wallet.address(), &expected_address);
    }

    #[test]
    fn test_from_seed() {
        let seed = [
            68, 49, 179, 87, 16, 197, 157, 230, 143, 50, 126, 44, 68, 63, 140, 252, 55, 165, 0, 21,
            89, 117, 205, 207, 77, 189, 251, 141, 193, 131, 96, 54,
        ];

        let wallet = MoneroWallet::from_seed(&Network::Mainnet, seed);

        assert_eq!(
            &wallet.address,
            "444SttAXhqVUeqCpPiTTwHcYTGpApapxjgsBzqSxWANvfmxGHJTLTTE6JgYM6yYNjmGBrGLhsVu8ZJh6RnFP8yqM6F8Lmb1"
        );
        assert_eq!(
            &wallet.private_spend_key(),
            "7db5d140c19b66de0c5c9743a851efbd37a500155975cdcf4dbdfb8dc1836006",
        );
        assert_eq!(
            &wallet.private_view_key(),
            "2d4f19ff6f2a2b2e918996e7ebbbabba791c29120ceb66518a4f232f3a88e30a",
        );
        assert_eq!(
            &wallet.public_spend_key(),
            "404e19168faffca55272b37aff9494d47e54fa7fd6ed34ee56db492a057953e7",
        );
        assert_eq!(
            &wallet.public_view_key(),
            "d22475de8a58511fb7362dc18fc79c5acc2848cbd73cc669c4e93811465e4c2e",
        );
    }
}