extern crate jsonrpc;

pub fn get_block() -> String {
    let block = jsonrpc::request("eth_blockNumber".to_string(), "[]".to_string());

    block.unwrap()
}
