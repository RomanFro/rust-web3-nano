extern crate jsonrpc;

pub fn block_number() -> String {
    let block_number = jsonrpc::request("eth_blockNumber".to_string(), "[]".to_string());

    block_number.unwrap()
}

pub fn get_block_by_number(params: String) -> String {
    let block = jsonrpc::request("eth_getBlockByNumber".to_string(), params);

    block.unwrap()
}
