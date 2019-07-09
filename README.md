<h1 align="center">wagu</h1>

<p align="center">
    <a href="https://travis-ci.com/ArgusDeveloper/wagu"><img src="https://travis-ci.com/ArgusDeveloper/wagu.svg"></a>
    <a href="https://crates.io/crates/wagu"><img src="https://img.shields.io/crates/v/wagu.svg"></a>
    <a href="./AUTHORS"><img src="https://img.shields.io/badge/authors-Argus-orange.svg"></a>
    <a href="./LICENSE-APACHE"><img src="https://img.shields.io/badge/license-APACHE-blue.svg"></a>
    <a href="./LICENSE-MIT"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
</p>

Wagu (pronounced  [wagyu](https://en.wikipedia.org/wiki/Wagyu)) is a **wa**llet **g**eneration **u**tility for cryptocurrencies.

## <a name='TableofContents'></a>Table of Contents

* [1. Overview](#1-overview)
* [2. Build Guide](#2-build-guide)
    * [2.1 Install Rust](#21-install-rust)
    * [2.2a Build from Crates.io](#22a-build-from-cratesio)
    * [2.2b Build from Source Code](#22b-build-from-source-code)
* [3. Features](#3-features)
	* [3.1 Generate a wallet with default options](#31-generate-a-wallet-with-default-options)
	* [3.2 Generate a mainnet or testnet wallet](#32-generate-a-mainnet-and-testnet-wallet)
	* [3.3 Generate a wallet as a JSON object](#33-generate-a-wallet-as-a-json-object)
	* [3.4 Generate multiple wallets of the same type](#34-generate-multiple-wallets-of-the-same-type)
	* [3.5 Generate a P2SH_P2WPKH SegWit wallet](#35-generate-a-p2sh_p2wpkh-segwit-wallet)
* [4. License](#4-license)

## 1. Overview

Wagu enables you to generate a wallet for the following cryptocurrencies:

* [Bitcoin](docs/bitcoin.md)
* [Ethereum](docs/ethereum.md)
* [Monero](docs/monero.md)
* [Zcash](docs/zcash.md)

Create a cryptocurrency wallet with the CLI as follows:

[![Wagu Bitcoin Demo](https://i.gyazo.com/134f7a29c4accef35ff730430cd87b52.gif)](https://gyazo.com/134f7a29c4accef35ff730430cd87b52)

## 2. Build Guide

### 2.1 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):  
  
  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

- Windows (32-bit):  
  
  Download the [Windows 32-bit executable](https://win.rustup.rs/i686) and follow the on-screen instructions.

### 2.2a Build from Crates.io

We recommend installing `wagu` this way. In your terminal, run:

```bash
cargo install wagu
```

Now to use `wagu`, in your terminal, run:
```bash
wagu
```
 
### 2.2b Build from Source Code

Alternatively, you can install `wagu` by building from the source code as follows:

```bash
# Download the source code
git clone https://github.com/ArgusDeveloper/wagu
cd wagu

# Build in release mode
$ cargo build --release
```

This will generate an executable under the `./target/release` directory. To use wagu, run the following command:
```bash
./target/release/wagu
```

## 3. Features

The following demonstrates the functionality of `wagu`. All examples are for the Bitcoin blockchain and more specific exampls can be found in the `/docs` folder.

### 3.1 Generate a wallet with default options

Generate a compressed mainnet private key and address with the following command:

`wagu bitcoin`

```bash
╰─ wagu bitcoin

        Private Key:    L5hax5dZaByC3kJ4aLrZgnMXGSQReqRDYNqM1VAeXpqDRkRjX42H
        Address:        1uNM6oivjCJU2RcsNbfooVwcPjDRhjW7U
        Network:        Mainnet
        Compressed:     true
```

### 3.2 Generate a mainnet and testnet wallet

Generate a testnet private key and address with the following command:

`wagu bitcoin --network testnet`

```bash
╰─ wagu bitcoin --network testnet

        Private Key:    92Rk56bU8atxbM9mUyNJtijc8XFyw7UHrDaasyTzcn9iLn4M9Le
        Address:        myPXYe7NrVpq8oYBugTFtHwamejxB6wNC8
        Network:        Testnet
        Compressed:     false
```

### 3.3 Generate a wallet as a JSON object

Generate a compressed mainnet private key and address with the following command:

`wagu bitcoin -j` OR `wagu bitcoin --json`

```bash
╰─ wagu -j
[
  {
    "privateKey": {
      "wif": "L5hax5dZaByC3kJ4aLrZgnMXGSQReqRDYNqM1VAeXpqDRkRjX42H",
      "network": "Mainnet",
      "compressed": true
    },
    "address": {
      "address": "1uNM6oivjCJU2RcsNbfooVwcPjDRhjW7U",
      "network": "Mainnet",
      "address_type": "P2PKH"
    }
  }
]
```

### 3.4 Generate multiple wallets of the same type

Generates multiple wallets with the following command:

`wagu bitcoin --count 3` OR `wagu bitcoin -n 3`

```bash
╰─ wagu bitcoin -n 3

        Private Key:    L5hax5dZaByC3kJ4aLrZgnMXGSQReqRDYNqM1VAeXpqDRkRjX42H
        Address:        1uNM6oivjCJU2RcsNbfooVwcPjDRhjW7U
        Network:        Mainnet
        Compressed:     true


        Private Key:    L4uNhZS86VLiKKGZZGNxwP7s67EfYfQ7S9bNnVfVbU9GBVVo2xoD
        Address:        16sz5SMFeRfwaqY6wKzkiufwPmF1J7RhAx
        Network:        Mainnet
        Compressed:     true


        Private Key:    KyH2BrThuUnzSXxDrDxQbpK277HxZfwPxVaCs5cwbzDEVNno2nts
        Address:        17QAwDwsLpehmCqSQXdHZb8vpsYVDnX7ic
        Network:        Mainnet
        Compressed:     true
```

### 3.5 Generate a P2SH_P2WPKH SegWit wallet

Generate a SegWit mainnet private key and address with the following command:

`wagu bitcoin --segwit`

```bash
╰─ wagu --segwit

        Private Key:    L13EzQBa7izHyXHdhAwBzApAPL1Q8rdVRpY7CASWXyFPyHTuPJxs
        Address:        3Qz5gtJ4GKoeSHHErF8Nvs9bDp5TQDw89o
        Network:        Mainnet
        Compressed:     true
```

## 4. License

This work is licensed under either of the following licenses, at your discretion.

- Apache License Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
