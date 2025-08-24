use sha2::{Digest, Sha256};

type Hash = [u8; 32];

fn hash_bytes(bytes: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
} // ai generated hash function

fn hash_pair(left: &Hash, right: &Hash) -> Hash {
    let left_str = hash_2_string(left);
    let right_str = hash_2_string(right);
    let combined = left_str + &right_str;
    hash_bytes(combined.as_bytes())
} // ai generated hash function

pub fn hash_2_string(hash: &Hash) -> String {
    hash.iter().map(|byte| format!("{:02x}", byte)).collect()
}

pub struct Merkle {
    depth: usize,
    length: usize,
    levels: Vec<Vec<Hash>>,
    levels_zero: Vec<Hash>,
}

impl Merkle {
    pub fn new(depth: usize) -> Self {
        let length = depth + 1;
        let levels = vec![vec![]; length];
        let mut levels_zero = vec![];
        for i in 0..length {
            if i == 0 {
                levels_zero.push(hash_bytes("".as_bytes())); // This value is preffered, for visual debugging using https://merkle-tree-visualizer.vercel.app/
            } else {
                levels_zero.push(hash_pair(&levels_zero[i - 1], &levels_zero[i - 1]));
            }
        }
        Self {
            depth: depth,
            length: length,
            levels: levels,
            levels_zero: levels_zero,
        }
    }

    pub fn add_leaf(&mut self, bytes: &[u8]) {
        if bytes == "".as_bytes() {
            println!("ERROR: Adding a empty leaf is not allowed");
            return;
        }
        if self.levels[0].len() >= 2_usize.pow(self.depth as u32) {
            println!("ERROR: All the leaves are already added");
            return;
        }
        // If true: in the previous level the element was added in the left
        // If false: in the previous level the element was added in the right
        let mut is_left = false;
        // If true: in the previous level a element was added
        // If false: in the previous level a element was updated
        let mut is_added = false;
        // If true: add in the current level a new element
        // If false: update in the current level a new element
        let mut add = false;
        let (mut cur_len, mut prev_i, mut hash_left, mut hash_right);
        for i in 0..self.length {
            cur_len = self.levels[i].len();
            // Leaves level
            if i == 0 {
                self.levels[i].push(hash_bytes(bytes));
            }
            // Other levels
            else {
                prev_i = i - 1;
                if is_left {
                    hash_left = self.levels[prev_i][self.levels[prev_i].len() - 1]; // Left member added
                    hash_right = self.levels_zero[prev_i];
                    if is_added {
                        add = true;
                    }
                } else {
                    hash_left = self.levels[prev_i][self.levels[prev_i].len() - 2]; // Right member added
                    hash_right = self.levels[prev_i][self.levels[prev_i].len() - 1];
                }
                if add {
                    self.levels[i].push(hash_pair(&hash_left, &hash_right));
                } else {
                    self.levels[i][cur_len - 1] = hash_pair(&hash_left, &hash_right);
                }
            }
            is_added = cur_len != self.levels[i].len();
            is_left = self.levels[i].len() % 2 == 1;
            add = false;
        }
        self.print_tree();
    }

    pub fn get_num_leaves(&self) -> usize {
        self.levels[0].len()
    }

    pub fn get_proof(&self, leaf_j: usize) -> (Hash, Vec<Hash>) {
        if leaf_j >= self.levels[0].len() {
            println!("ERROR: Leaf index out of range");
            return (hash_bytes("".as_bytes()), vec![]);
        }
        if self.levels[self.depth].len() != 1 {
            println!("ERROR: Root not computed yet");
            return (hash_bytes("".as_bytes()), vec![]);
        }
        let mut leaf_hash = self.levels_zero[0];
        let mut proof = vec![];
        let mut j = 0;
        let (mut is_left, mut j_other);
        for i in 0..self.length - 1 {
            // Leaves level
            if i == 0 {
                leaf_hash = self.levels[0][leaf_j];
                j = leaf_j;
            }
            // Other levels
            else {
                j = j / 2;
            }
            is_left = j % 2 == 0;
            if is_left {
                j_other = j + 1;
            } else {
                j_other = j - 1;
            }
            if self.levels[i].get(j_other).is_some() {
                proof.push(self.levels[i][j_other]);
            } else {
                proof.push(self.levels_zero[i]);
            }
        }
        self.print_proof(&proof);
        return (leaf_hash, proof);
    }

    pub fn get_root(&self) -> Hash {
        if self.levels[self.depth].len() != 1 {
            println!("ERROR: Root not computed yet");
            return hash_bytes("".as_bytes());
        }
        let root = self.levels[self.depth][0];
        println!("---------------------------------------------");
        println!("Root: {}", hash_2_string(&root));
        root
    }

    fn print_tree(&self) {
        println!("---------------------------------------------");
        println!("Tree:");
        for (level_idx, level) in self.levels.iter().enumerate() {
            println!("Level {}:", level_idx);
            for (node_idx, node) in level.iter().enumerate() {
                println!("  Node {}: {}", node_idx, hash_2_string(node));
            }
        }
    }

    fn print_proof(&self, proof: &Vec<Hash>) {
        println!("---------------------------------------------");
        println!("Proof:");
        for (idx, hash) in proof.iter().enumerate() {
            println!("  Proof {}: {}", idx, hash_2_string(hash));
        }
    }
}
