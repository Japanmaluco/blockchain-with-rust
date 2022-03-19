use rustchainlib::*;

fn main() {
    let mut blockchain = Blockchain::new();

    let genesis_block = Block::new(
        "0".to_owned(),
        vec![Transaction {
            sender: String::from("Japa"),
            receiver: String::from("Juka"),
            amount: 2000.0,
        }],
    );

    let second_block = Block::new(
        genesis_block.hash.to_owned(),
        vec![Transaction {
            sender: String::from("Juka"),
            receiver: String::from("Doskya"),
            amount: 1000.0,
        }],
    );

    blockchain.add_block(genesis_block);
    blockchain.add_block(second_block);
    println!("{:#?}", blockchain);
}
