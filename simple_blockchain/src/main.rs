/*
=> This Rust program implements a simple version of a blockchain.

=> The 'Block' struct reps a block in the blockchain.
It contains an index, a timestamp, data, a hash of its own contents, and the hash of the previous block. 
The 'new_block' function is used to create new blocks, calculating their hash using the 'calculate_hash' function.

=> N.B! I added a mine functionality to allow me to add new blocks. 

Made by Cyndie :)  
*/



// Import SystemTime and UNIX_EPOCH from the std::time module
// These will be used to create timestamps for each block
use std::time::{SystemTime, UNIX_EPOCH};   
use sha2::{Sha256, Digest}; //for sha256 hashing

// I set the difficulty of my Proof of Work(POW) algorithm to 4
const DIFFICULTY: usize = 4;  

#[derive(Debug)]

//the structure of our blockchain
struct Block {
    index: u32,
    timestamp: u64,
    data: String,
    current_hash: String,
    previous_hash: String,
    nonce: u64,  // Add a nonce to your block
}

impl Block {
    fn new_block(index: u32, timestamp: u64, data: String, previous_hash: String) -> Block {
        let mut block = Block {
            index,
            timestamp,
            data,
            current_hash: String::new(),  // Will be set in `mine`
            previous_hash,
            nonce: 0,  // Initial nonce value
        };
        block.mine();
        block
    }

    //function to calculate the block's hash
    fn calculate_hash(&self) -> String {
        let input = format!("{}  {}  {}  {}  {}", self.index, self.timestamp, self.data, self.previous_hash, self.nonce);
        let mut s = Sha256::new();
        s.update(input);
        let result = s.finalize();
        format!("{:x}", result)
    }

    // Mining function 
    // Adds nonce and calculate block's hash till it finds a hash that will satisfy our difficulty level.
    fn mine(&mut self) {
        //loop iterating from 0 to max u64 value finding nonce to solve our puzzle
        for nonce_attempt in 0..(u64::MAX) { 
            self.nonce = nonce_attempt;  //each iteration nonce attempt is set as current nonce
            let hash = self.calculate_hash();  // we calc hash using the set nonce
            if self.is_hash_valid(&hash) {  //we then check if the hash is valid
                self.current_hash = hash;  // if hash valid, its set to the current block
                return;
            }
        }
    }


    //function to check whether the hash calculated with the attempted nonce is valid or not
    fn is_hash_valid(&self, hash: &String) -> bool {
        &hash[0..DIFFICULTY] == "0".repeat(DIFFICULTY) //checks if our hash has 4 0s as per difficulty
    }
}

fn main() {
    let mut blocks = Vec::new();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let previous_hash = "0".to_string();
    let first_block = Block::new_block(0, timestamp, "First Block".to_string(), previous_hash);

    blocks.push(first_block);
    println!("{:?}", blocks[0]);  

    for i in 1..10 {
        println!("Type 'exit' to quit");

        println!("Please enter data for block number {}: ", i);

        let mut data = String::new();
        std::io::stdin().read_line(&mut data).expect("Unable to read your input");
        let data = data.trim().to_string();

        if data.trim() == "exit" {
            println!("See yah later!");
            break;
        }

        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = blocks[(i-1) as usize].current_hash.clone();
        let new_block = Block::new_block(i, timestamp, data, previous_hash);
        blocks.push(new_block);

        println!("{:?}", blocks[i as usize]); 
    }
}

