use ethers::core::types::Address;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Alias(std::collections::HashMap<String, String>);
#[allow(dead_code)]
pub fn alias_token(token_add: &String) -> Result<Address, Box<dyn std::error::Error>> {
    let file = File::open("alias/alias_token.json")?;
    let reader = BufReader::new(file);

    let tokens: Alias = serde_json::from_reader(reader)?;
    let mut token = None;

    for (name, address) in &tokens.0 {
        if token_add == name {
            token = Some(address.parse::<Address>()?);
            break;
        }
    }

    match token {
        Some(t) => Ok(t),
        None => token_add
            .parse::<Address>()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
    }
}
#[allow(dead_code)]
pub fn alias_address(router_add: &String) -> Result<Address, Box<dyn std::error::Error>> {
    let file = File::open("alias/alias_address.json")?;
    let reader = BufReader::new(file);

    let addresses: Alias = serde_json::from_reader(reader)?;
    let mut router = None;

    for (name, address) in &addresses.0 {
        if router_add == name {
            router = Some(address.parse::<Address>()?);
            break;
        }
    }

    match router {
        Some(t) => Ok(t),
        None => router_add
            .parse::<Address>()
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>),
    }
}
