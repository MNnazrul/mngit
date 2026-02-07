use sha2::{Digest, Sha256};

fn get_hash_in_hex(value: &str) -> String {
    let mut hasher = Sha256::new();
    let data = value.as_bytes();
    hasher.update(data);
    let hash = hasher.finalize();
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let answer = get_hash_in_hex("hello");
    println!("{}", answer);
    println!("{}", answer.len());
}
