use std::io;

pub fn request(method: String, params: String) -> Result<String, io::Error> {
    let body = format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\",\"params\":{}}}", method, params);
    let client = reqwest::Client::new();
    let res = client.post("https://mainnet.infura.io")
        .body(body)
        .send();
    Ok(res.unwrap().text().unwrap())
}
