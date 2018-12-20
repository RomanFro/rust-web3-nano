mod eth;

use std::time::{Instant};

fn main() {
    let now = Instant::now();

    let block_number = eth::block_number();

    println!("eth_blockNumber ({:?}): {:?}", now.elapsed(), block_number);

    let block = eth::get_block_by_number("[\"latest\",false]".to_string());

    println!("eth_getBlockByNumber ({:?}): {:?}", now.elapsed(), block);
}
