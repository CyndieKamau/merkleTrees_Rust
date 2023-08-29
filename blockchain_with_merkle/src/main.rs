use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io;


const DIFFICULTY: usize = 4;

#[derive(Clone, Debug)]

struct MerkleNode {

     left: Option<Box<MerkleNode>>,

     right: Option<Box<MerkleNode>>,

     data: String
    
}


impl MerkleNode {

    fn new(data: &str) -> Self {

        MerkleNode {

            left: None,

            right: None,

            data: data.to_string(),

        }

    }


    fn combine(left: &MerkleNode, right: &MerkleNode) -> Self {

        let combined_data = format!("{}{}", left.data, right.data);

        let mut hasher = Sha256::new();

        hasher.update(combined_data.as_bytes());

        let data = format!("{:x}", hasher.finalize());


        MerkleNode {

            left: Some(Box::new(left.clone())),

            right: Some(Box::new(right.clone())),

            data,

        }

    }

    fn leaf(data: &str) -> Self {

        let mut hasher = Sha256::new();

        hasher.update(data.as_bytes());

        let hashed_data = format!("{:x}", hasher.finalize());


        MerkleNode {

            left: None,

            right: None,

            data: hashed_data,
        }

    }


}

#[derive(Debug)]

struct MerkleTree {

    root: Option<MerkleNode>,

}


impl MerkleTree {

    fn new(transactions: Vec<&str>) -> Self {

        let mut nodes = Vec::new();

        for transaction in transactions {

            nodes.push(MerkleNode::leaf(transaction));
        }

        while nodes.len() > 1 {

            let mut new_level = Vec::new();

            while let Some(left_node) = nodes.pop() {

                if let Some(right_node) = nodes.pop() {

                    new_level.push(MerkleNode::combine(&left_node, &right_node));

                } else {

                    new_level.push(MerkleNode::combine(&left_node, &left_node));
                }
            }

            nodes = new_level;
        }

        MerkleTree {

            root: nodes.pop(),
        }
    }

    fn root(&self) -> Option<String> {


        self.root.as_ref().map(|node| node.data.clone())
    } 

}



#[derive(Debug)]

struct Block {

    index: u32,

    timestamp: u64,

    merkle_tree: MerkleTree,

    current_hash: String,

    previous_hash: String,

    nonce: u64,

}


impl Block {

    fn new_block(index: u32, timestamp: u64, transactions: Vec<&str>, previous_hash: String) -> Block {

        let mut block = Block {

            index,
            timestamp,
            merkle_tree: MerkleTree::new(transactions),
            current_hash: String::new(),
            previous_hash,
            nonce: 0,

        };

        block.mine();
        block
    }

    fn calculate_hash(&self) -> String {

        let input = format!("{} {} {} {} {}", self.index, self.timestamp, self.merkle_tree.root().unwrap_or_default(), self.previous_hash, self.nonce);

        let mut s = Sha256::new();

        s.update(input);

        let result = s.finalize();

        format!("{:x}", result)  

    }


    fn mine(&mut self) {

        for nonce_attempt in 0..(u64::MAX) {

            self.nonce = nonce_attempt;

            let hash = self.calculate_hash();

            if self.is_hash_valid(&hash) {

                self.current_hash = hash;
                return;
            }
        }
    }

    
    fn is_hash_valid(&self, hash: &String) -> bool {

        &hash[0..DIFFICULTY] == "0".repeat(DIFFICULTY)

    }
}


fn main() {

    let mut blocks = Vec::new();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let previous_hash = "0".to_string();

    let first_transactions = vec!["First Transaction"];
    let first_block = Block::new_block(0, timestamp, first_transactions, previous_hash);

    blocks.push(first_block);
    println!("{:?}", blocks[0]);

    for i in 1..10 {

        println!("Type 'exit' to quit.");
        println!("Enter transactions separated by ',' for block number {}: ", i);

        let mut data = String::new();
        io::stdin().read_line(&mut data)
            .expect("Failed to read input");
        let transactions: Vec<&str> = data.trim().split(',').collect();


        if transactions.contains(&"exit") {

            println!("See yah later!");
            break;
        }

        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let previous_hash = blocks[(i-1) as usize].current_hash.clone();
        let new_block = Block::new_block(i, timestamp, transactions, previous_hash);
        blocks.push(new_block);

        println!("{:?}", blocks[i as usize]); 
        
 
    }


}


