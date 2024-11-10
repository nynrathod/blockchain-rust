use blake2::Blake2b512;
use blake2::Digest;

// Define a trait for objects that can be hashed
pub trait Hashable {
    #[allow(dead_code)] // This will silence the warning about the unused `bytes` method
    fn bytes(&self, timestamp: i64, prev_hash: &str, nonce: u64) -> Vec<u8>;

    fn generate_hash(timestamp: i64, prev_hash: &str, nonce: u64) -> String {
        let mut hasher = Blake2b512::new();
        let mut bytes = Vec::new();

        // Append timestamp
        bytes.extend(&timestamp.to_be_bytes());

        // Append prev_hash
        bytes.extend(prev_hash.as_bytes());

        // Append nonce
        bytes.extend(&nonce.to_be_bytes());

        // You can add more fields here as needed

        hasher.update(&bytes);
        format!("{:x}", hasher.finalize())
    }
}
