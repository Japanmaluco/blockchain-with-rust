pub struct Block {
        pub hash: String,
        pub pre_hash: String,
        pub transaction: Vec<Transaction>,
        pub nonce: u64,
    }