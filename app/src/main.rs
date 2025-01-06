mod alias;
mod args;
mod client;
mod loader;
mod read;
mod tx;
mod utils;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Value {
    #[clap(subcommand)]
    command: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    //READ
    Balance,
    Balanceadd(args::Wethaddargs),
    Metadata(args::Tokenargs),
    Balanceof(args::Tokenargs),
    Balanceofadd(args::Tokenaddargs),
    Allowance(args::Allowanceargs),
    Wallet,
    //TRANSACTION
    Wrap(args::Wrapargs),
    Unwrap(args::Wrapargs),
    Approve(args::Approveargs),
    Swapeth(args::Swapethargs),
    Swaptoken(args::Swaptokens),
    Transfereth(args::Transfereth),
    Transfertoken(args::Transfertoken),
}

fn main() {
    let value = Value::parse();
    match &value.command {
        //READ
        Commands::Balance => {
            read::balance().ok();
        }
        Commands::Balanceadd(args::Wethaddargs { recipient }) => {
            read::balance_add(recipient).ok();
        }
        Commands::Metadata(args::Tokenargs { token }) => {
            read::metadata(token).ok();
        }
        Commands::Balanceof(args::Tokenargs { token }) => {
            read::balance_of(token).ok();
        }
        Commands::Balanceofadd(args::Tokenaddargs { token, recipient }) => {
            read::balance_of_add(token, recipient).ok();
        }
        Commands::Allowance(args::Allowanceargs { token_in, spender }) => {
            read::allowance(token_in, spender).ok();
        }
        Commands::Wallet => {
            read::wallet().ok();
        }
        //TRANSACTION
        Commands::Swapeth(args::Swapethargs {
            token_out,
            amount,
            slippage,
        }) => {
            tx::swap_eth(token_out, amount, slippage).ok();
        }
        Commands::Approve(args::Approveargs {
            token_in,
            spender,
            amount,
        }) => {
            tx::approve(token_in, spender, amount).ok();
        }
        Commands::Swaptoken(args::Swaptokens {
            token_in,
            token_out,
            amount,
            slippage,
        }) => {
            tx::swap_tokens(token_in, token_out, amount, slippage).ok();
        }
        Commands::Wrap(args::Wrapargs { amount }) => {
            tx::wrap(amount).ok();
        }
        Commands::Unwrap(args::Wrapargs { amount }) => {
            tx::unwrap(amount).ok();
        }
        Commands::Transfereth(args::Transfereth { amount, recipient }) => {
            tx::transfer_eth(amount, recipient).ok();
        }
        Commands::Transfertoken(args::Transfertoken {
            token,
            amount,
            recipient,
        }) => {
            tx::transfer_token(token, amount, recipient).ok();
        }
    }
}
