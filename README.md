# Simple Rust Blockchain with PoW and Merkle Trees

This repository contains two implementations of a simple blockchain system built in Rust:

### 1. A basic blockchain with a Proof of Work consensus algorithm.

### 2. An extension of the basic blockchain with the integration of Merkle Trees for transaction verification.


# Introduction

Blockchain technology forms the backbone of cryptocurrencies like Bitcoin and Ethereum. 

This project serves as a simple, educational representation of how blockchains function at their core, 

including mechanisms like PoW (Proof of Work) and Merkle Trees.


# Getting Started

## Prerequisites

Have Rust, Cargo installed in your system.


## Clone

```sh
git clone https://github.com/CyndieKamau/merkleTrees_Rust.git

cd merkleTrees_Rust

```

## Running the Program

For the Basic Blockchain;


```sh
cd simple_blockchain

cargo run

```

Successful running should output this;

```sh
hp@Cyndie:~/Desktop/merkleTrees_Rust/simple_blockchain/src$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `/home/hp/Desktop/rust/mine_new_block/target/debug/mine_new_block`
Block { index: 0, timestamp: 1693305314, data: "First Block", current_hash: "000059e58710beec19937b3c462fc09af3e1ed0e949c1cebeae496ee95804b54", previous_hash: "0", nonce: 27818 }
Type 'exit' to quit
Please enter data for block number 1: 
this is the second block
Block { index: 1, timestamp: 1693305326, data: "this is the second block", current_hash: "000059cfb8730d512eb4a44c58b6fe36b9c2f57852dbfe0aa8440ff4df0fd646", previous_hash: "000059e58710beec19937b3c462fc09af3e1ed0e949c1cebeae496ee95804b54", nonce: 170784 }
Type 'exit' to quit
Please enter data for block number 2: 
exit
See yah later!
```

For running the Merkle Tree program;

```sh
cd blockchain_with_merkle

cargo run

```

Successful program looks like this;

```sh
Finished dev [unoptimized + debuginfo] target(s) in 0.54s
     Running `/home/hp/Desktop/merkleTree/blockchain_with_merkle/target/debug/blockchain_with_merkle`
Block { index: 0, timestamp: 1693314005, merkle_tree: MerkleTree { root: Some(MerkleNode { left: None, right: None, data: "5f32a9be70edf223ccdda396a87a7ce72d38e773f5c83b37605e36f0efbc8ece" }) }, current_hash: "0000569630444c7a58e7510fe8b07d594a38f3226f02286344f45df22afe2108", previous_hash: "0", nonce: 53387 }
Type 'exit' to quit.
Enter transactions separated by ',' for block number 1: 
this is my first transaction, this is my second transaction, this is my third transaction
Block { index: 1, timestamp: 1693314061, merkle_tree: MerkleTree { root: Some(MerkleNode { left: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "bc4d20e6e4b23cf4abec7672e96dc6f96bb6b0fd77cd2af2d1dab3d819e5fabe" }), right: Some(MerkleNode { left: None, right: None, data: "bc4d20e6e4b23cf4abec7672e96dc6f96bb6b0fd77cd2af2d1dab3d819e5fabe" }), data: "af326c5dba7dac555c9639ff937d2efec565674a4c392507ad701f1b56f99ba3" }), right: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "27280c51ab40aa63515e90cecfd17cb3e4ffae2e3ebaa748ddb2f18d37651ce3" }), right: Some(MerkleNode { left: None, right: None, data: "edbc1656f117f39929afe6b11f4529797729f4e97b7cd6330531263402a95c45" }), data: "62a93d9854c13bb0699ad2626130b3ae538ccc48856b2588e4952db76827c947" }), data: "0719abe55019631aa71bf8186ddfbd926edcb7c74c51a14572bcc0db404f6f09" }) }, current_hash: "00006f41e173b76ac793c858939d04efa9d93ba4a4d7438cab60bd15dde5417a", previous_hash: "0000569630444c7a58e7510fe8b07d594a38f3226f02286344f45df22afe2108", nonce: 59897 }
Type 'exit' to quit.
Enter transactions separated by ',' for block number 2: 
second transaction, third transaction, fourth transaction
Block { index: 2, timestamp: 1693331831, merkle_tree: MerkleTree { root: Some(MerkleNode { left: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "2ae63ab0c786494e154c58d766f7478d21f305b0245d550605c0d7c3cc7c8843" }), right: Some(MerkleNode { left: None, right: None, data: "2ae63ab0c786494e154c58d766f7478d21f305b0245d550605c0d7c3cc7c8843" }), data: "172f161678e75c3c2e7fd4b77569462d2030b3d8abdab512f097c7ec53dc11d7" }), right: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "bd2facd9584174b59f7c531e37275159688e5a60c02e78c5d5d4be1ed5792d96" }), right: Some(MerkleNode { left: None, right: None, data: "749effdbf496b3d4b516641cc641ff140f89593f8d3421f730633a025cbdbef9" }), data: "805d6445fd8640d3f16dc98fc655428273b8f8611a8519b71cf5c5934517b1f9" }), data: "0e62d5801c615bb8b0529ffbc998a1f1baa855ad4262068ba405565c88fbb4eb" }) }, current_hash: "00005df5681086ab69348fe3b3e0bed97421910d83375b2ea02e273659a5a01a", previous_hash: "00006f41e173b76ac793c858939d04efa9d93ba4a4d7438cab60bd15dde5417a", nonce: 165510 }

```

# Basic Blockchain

This implementation is a straightforward representation of a blockchain system. It features:

* Block creation with index, timestamp, data, and previous hash.

* A Proof of Work consensus algorithm with a set difficulty of 4.

* SHA256 hashing for block verification.


# Blockchain with Merkle Trees

Building on the basic blockchain, this version integrates Merkle Trees to handle multiple transactions in each block.

Features include:

* Block creation with multiple transactions separated by a comma ','.

* Efficient and verifiable transaction handling with Merkle Trees.

* All features from the basic blockchain.

Here's a simple illustration showing a respective Merkle tree for Block number 8;

![Merkle Trees](/home/hp/Downloads/Merkle Trees.png)

Block 8;

```sh

Type 'exit' to quit.
Enter transactions separated by ',' for block number 8: 
25th tx, 26th tx, 27th tx, 28th tx
Block { index: 8, timestamp: 1693332033, merkle_tree: MerkleTree { root: Some(MerkleNode { left: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "61ef6ea548ce403c478964519a8eeacdf0406f21636bb85afd58bb065df5bedd" }), right: Some(MerkleNode { left: None, right: None, data: "ccdb02374a3a4e5f0ac3bdd651876f8395f57b3935c89c9ef4ef8cc8e248eb23" }), data: "1b71f51107ff6acaabed3102f498d77655f617ad339d42b1f480306a63502e84" }), right: Some(MerkleNode { left: Some(MerkleNode { left: None, right: None, data: "c33b22765a6480104048f022be86a0d765ac86b634676272fd1cb8453f33c44b" }), right: Some(MerkleNode { left: None, right: None, data: "bc3bf4c5135ea276efe1f1474a1702a9d27e883af8f2bcf803b94975871afea3" }), data: "a36ef17b69605340139249b51f166225f3f5d9e0561e167e0a9257268c567de2" }), data: "6ca2ab3d74876dc8afa3f5e10b08805837e9d62034835697e4fe8fe89f4687be" }) }, current_hash: "0000fd017f2ef866369302f0bedce28efddf5ffb6954e6f74e93d3d532e0f4f7", previous_hash: "0000fba3713ea65377c201be8dc462f0de5924510e40c22302e212641cd32ef8", nonce: 27870 }

```
