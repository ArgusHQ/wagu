use super::*;
use crate::address::Format;
use wagu_model::{AddressError, Network, NetworkError, PrivateKeyError};

use serde::Serialize;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Testnet;

impl Network for Testnet {}

impl BitcoinNetwork for Testnet {
    /// Returns the address prefix of the given network.
    fn to_address_prefix(format: &Format) -> Vec<u8> {
        match format {
            Format::P2PKH => vec![0x6F],
            Format::P2SH_P2WPKH => vec![0xC4],
            Format::Bech32 => vec![0x74, 0x62]
        }
    }

    /// Returns the network of the given address prefix.
    fn from_address_prefix(prefix: &[u8]) -> Result<Self, AddressError> {
        match (prefix[0], prefix[1]) {
            (0x6F, _) | (0xC4, _) | (0x74, 0x62) => Ok(Self),
            _ => Err(AddressError::InvalidPrefix(prefix.to_owned()))
        }
    }

    /// Returns the wif prefix of the given network.
    fn to_wif_prefix() -> u8 {
        0xEF
    }

    /// Returns the network of the given wif prefix.
    fn from_wif_prefix(prefix: u8) -> Result<Self, PrivateKeyError> {
        match prefix {
            0xEF => Ok(Self),
            _ => Err(PrivateKeyError::InvalidPrefix(vec![prefix]))
        }
    }

    /// Returns the extended private key version bytes of the given network.
    /// https://github.com/satoshilabs/slips/blob/master/slip-0132.md
    fn to_extended_private_key_version_bytes(
        format: &Format
    ) -> Result<Vec<u8>, ExtendedPrivateKeyError> {
        match format {
            Format::P2PKH => Ok(vec![0x04, 0x35, 0x83, 0x94]), // tpriv
            Format::P2SH_P2WPKH => Ok(vec![0x04, 0x4A, 0x4E, 0x28]), // upriv
            _ => Err(ExtendedPrivateKeyError::UnsupportedFormat(format.to_string()))
        }
    }

    /// Returns the network of the given extended private key version bytes.
    /// https://github.com/satoshilabs/slips/blob/master/slip-0132.md
    fn from_extended_private_key_version_bytes(
        prefix: &[u8]
    ) -> Result<Self, ExtendedPrivateKeyError> {
        match prefix[0..4] {
            [0x04, 0x35, 0x83, 0x94] | [0x04, 0x4A, 0x4E, 0x28] => Ok(Self),
            _ => Err(ExtendedPrivateKeyError::InvalidVersionBytes(prefix.to_vec()))
        }
    }

    /// Returns the extended public key version bytes of the given network.
    /// https://github.com/satoshilabs/slips/blob/master/slip-0132.md
    fn to_extended_public_key_version_bytes(
        format: &Format
    ) -> Result<Vec<u8>, ExtendedPublicKeyError> {
        match format {
            Format::P2PKH => Ok(vec![0x04, 0x35, 0x87, 0xCF]), // tpub
            Format::P2SH_P2WPKH => Ok(vec![0x04, 0x4A, 0x52, 0x62]), // upub
            _ => Err(ExtendedPublicKeyError::UnsupportedFormat(format.to_string()))
        }
    }

    /// Returns the network of the given extended public key version bytes.
    /// https://github.com/satoshilabs/slips/blob/master/slip-0132.md
    fn from_extended_public_key_version_bytes(
        prefix: &[u8]
    ) -> Result<Self, ExtendedPublicKeyError> {
        match prefix[0..4] {
            [0x04, 0x35, 0x87, 0xCF] | [0x04, 0x4A, 0x52, 0x62] => Ok(Self),
            _ => Err(ExtendedPublicKeyError::InvalidVersionBytes(prefix.to_vec()))
        }
    }
}

impl FromStr for Testnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "testnet" => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into()))
        }
    }
}

impl fmt::Display for Testnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "testnet")
    }
}