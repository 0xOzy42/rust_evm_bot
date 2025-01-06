use colored::Colorize;
use ethers::{
    middleware::SignerMiddleware,
    prelude::k256::ecdsa::SigningKey,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
};
use eyre::Result;
use std::env;
use std::sync::Arc;
#[path = "alias.rs"]
mod alias;

pub async fn get_client() -> Result<Arc<Provider<Http>>> {
    //config
    dotenv::dotenv().ok();
    let client = Provider::<Http>::try_from(&env::var("RPC").unwrap())?;
    let client = Arc::new(client);

    Ok(Arc::clone(&client))
}
pub async fn get_client_signed() -> Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>>
{
    dotenv::dotenv().ok();
    let provider = Provider::<Http>::try_from(env::var("RPC").unwrap())?;
    let chain_id = provider.get_chainid().await?;
    let wallet = env::var("PRIVATE_KEY")
        .unwrap()
        .parse::<LocalWallet>()?
        .with_chain_id(chain_id.as_u64());
    let client: SignerMiddleware<
        Provider<Http>,
        ethers::signers::Wallet<ethers::core::k256::ecdsa::SigningKey>,
    > = SignerMiddleware::new(provider.clone(), wallet.clone());

    Ok(Arc::new(client))
}

pub async fn print_state(
    provider_signed: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) -> Result<()> {
    //print state Account/Bc
    let account = provider_signed.address();
    let block_number = provider_signed.get_block_number().await?;
    let nonce = provider_signed.get_transaction_count(account, None).await?;
    println!("{}", "\n===== INFOS =====\n".bold().magenta());
    println!("Account: {:?}", account);
    println!("Block: {:?}", block_number);
    println!("Gas price: {:?}", provider_signed.get_gas_price().await?);
    println!("Nonce: {:?}", nonce);
    Ok(())
}
