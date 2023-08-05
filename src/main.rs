pub mod block;
pub mod blockchain;
pub mod pow;

use hex;

fn main() {
    let mut bc = blockchain::BlockChain::new();

    bc.add_block("Send 1 BTC to Ivan");
    bc.add_block("Send 2 more BTC to Ivan");

    let mut iterator = bc.iterator();
    loop {
        match iterator.next() {
            Some(block) => {
                println!("Prev. hash: {}", hex::encode(block.prev_block_hash()));
                println!(
                    "Data: {}",
                    String::from_utf8(block.data().to_vec()).unwrap()
                );
                println!("Hash: {}", hex::encode(block.hash()));

                let pow = pow::ProofOfWork::new(&block, 8);
                println!("PoW: {}", pow.validate());

                println!("");
            }
            None => break,
        }
    }
}
