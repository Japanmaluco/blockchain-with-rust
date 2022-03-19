use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

struct Block {
    pub hash: [u8; 64],
    pub password: String,
}

struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

fn main() {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;

    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt);

    let password: String = "password".to_string();

    let mut hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    let mut prev_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    let blockchain = Block { hash, password };

    let users = Transaction {
        sender: String::from("Japa"),
        receiver: String::from("Juka"),
        amount: 2000.0,
    };

    println!(
        "Block Genesis:\nhash: {}\n{:?}",
        HEXUPPER.encode(&hash),
        blockchain.hash
    );

    println!(
        "\nSender: {}\nReceiver: {}\nAmount: {}",
        users.sender, users.receiver, users.amount
    );
}
