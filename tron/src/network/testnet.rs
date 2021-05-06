use crate::network::TronNetwork;
use wagyu_model::{ChildIndex, Network, NetworkError};

use serde::Serialize;
use std::{fmt, str::FromStr};

/// Represents an Tron main network.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Testnet;

impl Network for Testnet {
    const NAME: &'static str = "testnet";
}

impl TronNetwork for Testnet {
    const CHAIN_ID: u32 = 1;
    const NETWORK_ID: u32 = 1;
    const HD_COIN_TYPE: ChildIndex = ChildIndex::Hardened(195);

    fn address_prefix() -> u8 {
        0xa0
    }
}

impl FromStr for Testnet {
    type Err = NetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            Self::NAME => Ok(Self),
            _ => Err(NetworkError::InvalidNetwork(s.into())),
        }
    }
}

impl fmt::Display for Testnet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}