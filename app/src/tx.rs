use colored::Colorize;
use ethers::{
    contract::abigen,
    core::types::{Address, U256},
    providers::Middleware,
    types::Eip1559TransactionRequest,
};
use eyre::Result;
abigen!(IUniswapRouter, "./abi/router_univ2.json");
abigen!(IERC20, "./abi/erc20_abi.json");
abigen!(IWETH, "./abi/weth_abi.json");

use crate::{
    client::{get_client, get_client_signed, print_state},
    loader::{start_loader, stop_loader},
    utils::{get_token_metadata, get_valid_timestamp, to_human_readable},
};
#[path = "alias.rs"]
mod alias;

#[tokio::main]
pub async fn swap_tokens(
    token_in_a: &String,
    token_out_a: &String,
    amount: &String,
    slippage: &f64,
) -> Result<()> {
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();
    print_state(&client_signed).await?;

    println!("{}", "\n===== SWAP TOKENS =====\n".bold().blue());

    //Check for alias params
    let token_in = alias::alias_token(token_in_a).unwrap();
    let token_out = alias::alias_token(token_out_a).unwrap();
    let (decimals_token_in, symbol_token_in, _) = get_token_metadata(&token_in_a).await?;
    let (decimals_token_out, symbol_token_out, _) = get_token_metadata(&token_out_a).await?;

    //params
    let router_addr = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    let router = IUniswapRouter::new(router_addr, client_signed.clone());
    let token_contract = IERC20::new(token_in, client_signed.clone());

    let valid_timestamp = get_valid_timestamp(300000);
    let path = vec![token_in, token_out];

    let balance_of = token_contract.balance_of(account).call().await?;
    println!(
        "{} {} ({}) {}",
        "Actual Balance:".bold(),
        to_human_readable(balance_of, decimals_token_in),
        balance_of,
        symbol_token_in
    );

    let amount_in = if amount == "max" {
        balance_of
    } else {
        U256::from_dec_str(amount).unwrap()
    };

    if balance_of.is_zero() {
        println!("{}", "Balance is 0. Transaction cancelled.".red());
        return Ok(());
    }

    if amount_in > balance_of {
        println!(
            "{} {} ({}) {} ({})",
            "Amount".red(),
            amount_in,
            "is greater than the balance".red(),
            balance_of,
            "Transaction cancelled.".red()
        );
        return Ok(());
    }

    let amounts_out = router.get_amounts_out(amount_in, path).call().await?;

    let slippage_decimal = slippage / 100.0;
    let amount_out_min = amounts_out[1]
        - (amounts_out[1] * U256::from((slippage_decimal * 1e18) as u64) / U256::from(1e18 as u64));

    println!("Slippage: {} %", slippage);
    println!(
        "Swap {} {} to {} {}...",
        to_human_readable(amount_in, decimals_token_in),
        symbol_token_in,
        to_human_readable(amount_out_min, decimals_token_out),
        symbol_token_out
    );

    //check allowance and approve if necessary
    let allow = token_contract
        .allowance(account, router_addr)
        .call()
        .await?;
    if allow < amount_in {
        //let's do an approve
        println!(
            "\n{}",
            "Insufficient allowance, approve process incoming...".yellow()
        );
        // Start loader for approval
        let loader = start_loader("Approving tokens...");
        let receipt_approve = token_contract
            .approve(router_addr, amount_in)
            .send()
            .await?
            .await?;
        stop_loader(loader);
        let receipt_approve_json: serde_json::Value = serde_json::to_value(&receipt_approve)?;
        println!(
            "Successfully approved {:?} {}\n",
            to_human_readable(amount_in, decimals_token_in),
            symbol_token_in
        );
        println!(
            "Approve Receipt: {}",
            receipt_approve_json["transactionHash"]
                .as_str()
                .unwrap_or("Unknown hash")
        );
    }

    let loader = start_loader("Swapping tokens...");
    let receipt_swap = router
        .swap_exact_tokens_for_tokens(
            amount_in,
            amounts_out[1],
            vec![token_in, token_out],
            account,
            U256::from(valid_timestamp),
        )
        .send()
        .await?
        .await?;
    stop_loader(loader);

    let receipt_swap_json: serde_json::Value = serde_json::to_value(&receipt_swap)?;
    println!(
        "Swap Receipt: {}",
        receipt_swap_json["transactionHash"]
            .as_str()
            .unwrap_or("Unknown hash")
    );

    Ok(())
}
#[tokio::main]
pub async fn swap_eth(token_out_a: &String, amount: &String, slippage: &f64) -> Result<()> {
    println!("{}", "\n===== SWAP ETH TO TOKEN =====\n".bold().blue());

    let client = get_client().await?;
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();
    print_state(&client_signed).await?;

    println!("{}", "\n===== SWAP ETH =====\n".bold().blue());

    //params
    let router_addr = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D".parse::<Address>()?;
    let router = IUniswapRouter::new(router_addr, client_signed.clone());
    let token_in = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;
    //Check for alias params
    let token_out = alias::alias_token(token_out_a).unwrap();
    let (decimals_token_out, symbol_token_out, _) = get_token_metadata(&token_out_a).await?;

    let valid_timestamp = get_valid_timestamp(300000);
    let path = vec![token_in, token_out];

    let eth_balance = client.get_balance(account, None).await?;

    println!(
        "{} {}",
        "Actual ETH Balance:".bold(),
        to_human_readable(eth_balance, 18)
    );

    let amount_in = if amount == "max" {
        eth_balance
    } else {
        U256::from_dec_str(amount).unwrap()
    };

    if eth_balance.is_zero() {
        println!("{}", "ETH Balance is 0. Transaction cancelled.".red());
        return Ok(());
    }

    if amount_in > eth_balance {
        println!(
            "{} {} ({}) {} ({})",
            "Amount".red(),
            amount_in,
            "is greater than the balance".red(),
            eth_balance,
            "Transaction cancelled.".red()
        );
        return Ok(());
    }

    //AMOUNTOUT
    let amounts_out = router.get_amounts_out(amount_in, path).call().await?;

    let slippage_decimal = slippage / 100.0;
    let amount_out_min = amounts_out[1]
        - (amounts_out[1] * U256::from((slippage_decimal * 1e18) as u64) / U256::from(1e18 as u64));

    println!("Slippage: {} %", slippage);
    println!(
        "Swap {} ETH for {} {}...",
        to_human_readable(amount_in, 18),
        to_human_readable(amount_out_min, decimals_token_out),
        symbol_token_out
    );
    let loader = start_loader("Swapping tokens...");
    let receipt_swap = router
        .swap_exact_eth_for_tokens(
            amount_out_min,
            vec![token_in, token_out],
            account,
            U256::from(valid_timestamp),
        )
        .value(amount_in)
        .send()
        .await?
        .await?;
    stop_loader(loader);
    let receipt_swap_json: serde_json::Value = serde_json::to_value(&receipt_swap)?;
    println!(
        "Swap Receipt: {}",
        receipt_swap_json["transactionHash"]
            .as_str()
            .unwrap_or("Unknown hash")
    );

    Ok(())
}
#[tokio::main]
pub async fn approve(token_in_a: &String, spender_a: &String, amount: &String) -> Result<()> {
    let client_signed = get_client_signed().await?;
    print_state(&client_signed).await?;

    println!("{}", "\n===== APPROVE TOKEN =====\n".bold().blue());

    // Check for alias params
    let token_in = alias::alias_token(token_in_a).unwrap();
    let (decimals_token_in, symbol_token_in, _) = get_token_metadata(&token_in_a).await?;
    let spender = alias::alias_address(spender_a).unwrap();
    let token_contract = IERC20::new(token_in, client_signed);
    let amount_in = if amount == "max" {
        println!("Approve {} {} to {} ", "MAX", symbol_token_in, spender_a);
        U256::MAX
    } else {
        println!(
            "Approve {} {} to {} ",
            to_human_readable(U256::from_dec_str(amount).unwrap(), decimals_token_in),
            symbol_token_in,
            spender_a
        );
        U256::from_dec_str(amount).unwrap()
    };

    let loader = start_loader("Approving token...");
    let receipt_approve = token_contract
        .approve(spender, amount_in)
        .send()
        .await?
        .await?;
    stop_loader(loader);

    let receipt_approve_json: serde_json::Value = serde_json::to_value(&receipt_approve)?;
    println!(
        "Approve Receipt: {}",
        receipt_approve_json["transactionHash"]
            .as_str()
            .unwrap_or("Unknown hash")
    );

    Ok(())
}
#[tokio::main]
pub async fn transfer_eth(amount: &String, recipient_a: &String) -> Result<()> {
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();
    print_state(&client_signed).await?;
    println!("{}", "\n===== TRANSFER ETH =====\n".bold().blue());

    //Check for alias params
    let recipient = alias::alias_address(recipient_a).unwrap();

    let balance = client_signed.get_balance(account, None).await?;
    println!(
        "{} {} ETH ({} wei) \n",
        "Actual Balance:".bold(),
        to_human_readable(balance, 18),
        balance
    );

    //param
    let amount_in = if amount == "max" {
        balance
    } else {
        U256::from_dec_str(amount).unwrap()
    };

    if balance.is_zero() {
        println!("{}", "Balance is 0. Transaction cancelled.".red());
        return Ok(());
    }

    if amount_in > balance {
        println!(
            "{} {} {} {} ({})",
            "Amount".red(),
            amount_in,
            "is greater than the balance".red(),
            balance,
            "Transaction cancelled.".red()
        );
        return Ok(());
    }

    println!(
        "Transfer {} ETH to address {}",
        to_human_readable(amount_in, 18),
        recipient
    );
    ////////////////:
    let loader = start_loader("Transferring ETH...");
    let tx = Eip1559TransactionRequest::new()
        .to(recipient)
        .value(amount_in)
        .max_priority_fee_per_gas(U256::from(2000000000_u128)); // 2 Gwei
    let receipt_transfer = client_signed.send_transaction(tx, None).await?.await?;
    stop_loader(loader);
    let receipt_transfer_json: serde_json::Value = serde_json::to_value(&receipt_transfer)?;
    println!(
        "Transfer Receipt: {}",
        receipt_transfer_json["transactionHash"]
            .as_str()
            .unwrap_or("Unknown hash")
    );

    Ok(())
}

#[tokio::main]
pub async fn transfer_token(token_a: &String, amount: &String, recipient_a: &String) -> Result<()> {
    let client_signed = get_client_signed().await?;
    let account = client_signed.address();
    print_state(&client_signed).await?;
    println!("{}", "\n===== TRANSFER TOKEN =====\n".bold().blue());

    //Check for alias params
    let token = alias::alias_token(token_a).unwrap();
    let recipient = alias::alias_address(recipient_a).unwrap();
    let (decimals_token, symbol_token, _) = get_token_metadata(&token_a).await?;
    let token_contract = IERC20::new(token, client_signed);

    let balance_of = token_contract.balance_of(account).call().await?;
    println!(
        "{} {} ({}) {}\n",
        "Actual Balance:".bold(),
        to_human_readable(balance_of, decimals_token),
        balance_of,
        symbol_token
    );
    //param
    let amount_in = if amount == "max" {
        balance_of
    } else {
        U256::from_dec_str(amount).unwrap()
    };

    if balance_of.is_zero() {
        println!("{}", "Balance is 0. Transaction cancelled.".red());
        return Ok(());
    }

    if amount_in > balance_of {
        println!(
            "{} {} {} {} ({})",
            "Amount".red(),
            amount_in,
            "is greater than the balance".red(),
            balance_of,
            "Transaction cancelled.".red()
        );
        return Ok(());
    }

    //check allowance and approve if necessary
    let allow = token_contract.allowance(account, recipient).call().await?;
    if allow < amount_in {
        //let's do an approve
        let loader = start_loader("Insufficient allowance, approve process incoming...");
        let receipt_approve = token_contract
            .approve(recipient, amount_in)
            .send()
            .await?
            .await?;
        stop_loader(loader);
        let receipt_approve_json: serde_json::Value = serde_json::to_value(&receipt_approve)?;
        println!(
            "Successfully approved {:?} {}\n",
            to_human_readable(balance_of, decimals_token),
            symbol_token
        );
        println!(
            "Approve Receipt: {}",
            receipt_approve_json["transactionHash"]
                .as_str()
                .unwrap_or("Unknown hash")
        );
    }
    println!(
        "Transfer {:?} {} to address {:?}",
        amount_in, symbol_token, recipient
    );
    let loader = start_loader("Transferring token...");
    let receipt_transfer = token_contract
        .transfer(recipient, amount_in)
        .send()
        .await?
        .await?;
    stop_loader(loader);

    let receipt_transfer_json: serde_json::Value = serde_json::to_value(&receipt_transfer)?;
    println!(
        "Transfer Receipt: {}",
        receipt_transfer_json["transactionHash"]
            .as_str()
            .unwrap_or("Unknown hash")
    );

    Ok(())
}

#[tokio::main]
pub async fn wrap(amount: &String) -> Result<()> {
    let provider_signed = get_client_signed().await?;
    let account = provider_signed.address();
    print_state(&provider_signed).await?;

    //params
    let weth_addr = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;
    let weth_contract = IWETH::new(weth_addr, provider_signed);

    //param
    let amount_in;
    if amount == "max" {
        amount_in = weth_contract.balance_of(account).call().await?;
    } else {
        amount_in = U256::from_dec_str(amount).unwrap();
    }

    let receipt_wrap = weth_contract
        .deposit()
        .value(amount_in)
        .send()
        .await?
        .await?;
    println!("Wrap {:?} ethers into weth", amount_in);

    println!(
        "Transaction successful with hash: {:?}",
        receipt_wrap.unwrap().transaction_hash
    );

    Ok(())
}

#[tokio::main]
pub async fn unwrap(amount: &String) -> Result<()> {
    let provider_signed = get_client_signed().await?;
    let account = provider_signed.address();
    print_state(&provider_signed).await?;

    //params
    let weth_addr = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse::<Address>()?;
    let weth_contract = IWETH::new(weth_addr, provider_signed);

    //param
    let amount_in;
    if amount == "max" {
        amount_in = weth_contract.balance_of(account).call().await?;
    } else {
        amount_in = U256::from_dec_str(amount).unwrap();
    }

    let receipt_unwrap = weth_contract
        .deposit()
        .value(amount_in)
        .send()
        .await?
        .await?;
    println!("Unwrap {:?} weth into ether", amount_in);

    println!(
        "Transaction successful with hash: {:?}",
        receipt_unwrap.unwrap().transaction_hash
    );

    Ok(())
}
