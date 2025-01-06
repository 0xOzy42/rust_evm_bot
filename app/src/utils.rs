use ethers::{contract::abigen, core::types::U256};
use eyre::Result;
use std::time::{SystemTime, UNIX_EPOCH};
abigen!(IERC20, "./abi/erc20_abi.json");
use crate::client::get_client;
#[path = "alias.rs"]
mod alias;
trait ToF64 {
    fn as_f64(&self) -> f64;
}

impl ToF64 for U256 {
    fn as_f64(&self) -> f64 {
        let mut result = 0.0;
        let mut base = 1.0;

        for i in 0..4 {
            let word = self.0[i] as f64;
            result += word * base;
            base *= 2.0_f64.powi(64);
        }

        result
    }
}

pub fn to_human_readable(balance: U256, decimals: u8) -> f64 {
    let factor = U256::exp10(decimals as usize); // 10^decimals

    let balance_f64 = balance.as_f64();
    let factor_f64 = factor.as_f64();

    balance_f64 / factor_f64
}

pub async fn get_token_metadata(token_add: &String) -> Result<(u8, String, String)> {
    let client = get_client().await?;

    //Check for alias params
    let token = alias::alias_token(token_add).unwrap();

    let token_contract = IERC20::new(token, client);

    let decimals = token_contract.decimals().call().await?;

    let symbol: String = token_contract.symbol().call().await?;

    let name: String = token_contract.name().call().await?;

    Ok((decimals, symbol, name))
}

pub fn get_valid_timestamp(future_millis: u128) -> u128 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let time_millis = since_epoch.as_millis().checked_add(future_millis).unwrap();

    time_millis
}
