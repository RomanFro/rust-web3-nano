use std::io;

pub fn request(method: String, params: String) -> Result<String, io::Error> {
    let body = format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\",\"params\":{}}}", method, params);
    let client = reqwest::Client::new();

    let response = client.post("https://mainnet.infura.io")
        .body(body)
        .send();

    let parsed_response = json::parse(&response.unwrap().text().unwrap()).unwrap();
    let result = &parsed_response["result"];

    // TODO: add some checks

    Ok(result.to_string())
}
