use crate::alias::alias_address;
use crate::alias::alias_token;
use crate::client::get_client_signed;
use crate::utils::get_token_metadata;
use crate::utils::to_human_readable;
use colored::Colorize;
use ethers::{contract::abigen, providers::Middleware};
use eyre::Result;
#[path = "alias.rs"]
mod alias;

abigen!(IERC20, "./abi/erc20_abi.json");

#[tokio::main]
pub async fn balance_of(token_add: &String) -> Result<()> {
    println!("{}", "\n===== BALANCE OF TOKEN =====\n".bold().blue());
    //client
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();

    //Check for alias params
    let token = alias::alias_token(token_add).unwrap();
    let token_contract = IERC20::new(token, client_signed);
    let balance = token_contract.balance_of(account).call().await?;
    let (decimals_token, symbol_token, _) = get_token_metadata(&token_add).await?;
    println!(
        "Your Balance: \n{} {} ({} wei)",
        to_human_readable(balance, decimals_token),
        symbol_token,
        balance
    );

    Ok(())
}
#[tokio::main]
pub async fn balance_of_add(token_add: &String, recipient_add: &String) -> Result<()> {
    println!(
        "{}",
        "\n===== BALANCE OF TOKEN FOR ADDRESS=====\n".bold().blue()
    );
    //client
    let client_signed = get_client_signed().await?;
    //params
    let token = alias_token(token_add).unwrap();
    let recipient = alias_address(recipient_add).unwrap();

    let token_contract = IERC20::new(token, client_signed);
    let balance = token_contract.balance_of(recipient).call().await?;
    let (decimals_token, symbol_token, _) = get_token_metadata(&token_add).await?;
    println!(
        "Balance of {} :\n{} {} ({} wei)",
        recipient,
        to_human_readable(balance, decimals_token),
        symbol_token,
        balance
    );

    Ok(())
}

#[tokio::main]
pub async fn balance() -> Result<()> {
    println!("{}", "\n===== BALANCE ETH =====\n".bold().blue());
    //client
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();

    let balance = client_signed.get_balance(account, None).await?;
    println!(
        "Your ETH Balance :\n{} ETH ({} wei)",
        to_human_readable(balance, 18),
        balance
    );

    Ok(())
}
#[tokio::main]
pub async fn balance_add(recipient_add: &String) -> Result<()> {
    println!("{}", "\n===== BALANCE ETH OF ADDRESS=====\n".bold().blue());
    //client
    let client_signed = get_client_signed().await?;
    //params
    let recipient = alias_address(recipient_add).unwrap();

    let balance = client_signed.get_balance(recipient, None).await?;
    println!(
        "ETH Balance of {} :\n{} ETH ({} wei)",
        recipient,
        to_human_readable(balance, 18),
        balance
    );

    Ok(())
}
#[tokio::main]
pub async fn metadata(token_add: &String) -> Result<()> {
    println!("{}", "\n===== METADATA TOKEN=====\n".bold().blue());

    //Check for alias params
    let token = alias_token(token_add).unwrap();
    let (decimals_token, symbol_token, name_token) = get_token_metadata(&token_add).await?;
    println!("Address: {}", token);
    println!("Symbol: {}", symbol_token);
    println!("Name: {}", name_token);
    println!("Decimals : {}", decimals_token);

    Ok(())
}

#[tokio::main]
pub async fn allowance(token_a: &String, spender_a: &String) -> Result<()> {
    println!("{}", "\n===== ALLOWANCE =====\n".bold().blue());
    //client
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();

    //Check for alias params
    let token = alias::alias_token(token_a).unwrap();
    let spender = alias::alias_address(spender_a).unwrap();

    let token_contract = IERC20::new(token, client_signed);
    let allow = token_contract.allowance(account, spender).call().await?;
    println!("{} allowed to address {}", allow, spender);

    Ok(())
}

#[tokio::main]
pub async fn wallet() -> Result<()> {
    println!("{}", "\n===== WALLET =====\n".bold().blue());

    //client
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();
    let balance = client_signed.get_balance(account, None).await?;
    println!("Wallet: {}", account);
    println!(
        "ETH Balance :\n{} ETH ({} wei)",
        to_human_readable(balance, 18),
        balance
    );

    Ok(())
}
