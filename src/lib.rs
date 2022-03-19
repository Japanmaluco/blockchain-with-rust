extern crate time;

mod block;
pub use block::Block;
pub mod transaction;
pub use transaction::Transaction;
pub mod blockchain;
pub use blockchain::Blockchain;

pub fn now() -> i64 {
    time::now().to_timespec().sec
}

pub fn calculate_hash(pre_hash: &String, transaction: &Vec<Transaction>, timestamp: i64) -> String {
    let mut bytes = vec![];
    bytes.extend(&timestamp.to_ne_bytes());
    bytes.extend(
        transaction
            .iter()
            .flat_map(|transaction| transaction.bytes())
            .collect::<Vec<u8>>(),
    );

    bytes.extend(pre_hash.as_bytes());
    crypto_hash::hex_digest(crypto_hash::Algorithm::SHA256, &bytes)
}
