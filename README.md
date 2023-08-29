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

```
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

# Basic Blockchain

This implementation is a straightforward representation of a blockchain system. It features:

* Block creation with index, timestamp, data, and previous hash.

* A Proof of Work consensus algorithm with a set difficulty of 4.

* SHA256 hashing for block verification.


