use clap::Args;

////////////////////////////////////READ////////////////////////////////////
#[derive(Debug, Args)]
pub struct Tokenargs {
    #[arg(short = 'i', long = "intoken")]
    pub token: String,
}
#[derive(Debug, Args)]
pub struct Wethaddargs {
    #[arg(short = 'r', long = "recipient")]
    pub recipient: String,
}
#[derive(Debug, Args)]
pub struct Tokenaddargs {
    #[arg(short = 'i', long = "intoken")]
    pub token: String,
    #[arg(short = 'r', long = "recipient")]
    pub recipient: String,
}
#[derive(Debug, Args)]
pub struct Allowanceargs {
    #[arg(short = 'i', long = "intoken")]
    pub token_in: String,
    #[arg(short = 's', long = "spender")]
    pub spender: String,
}
////////////////////////////////////TRANSACTION////////////////////////////////////
#[derive(Debug, Args)]
pub struct Approveargs {
    #[arg(short = 'i', long = "intoken")]
    pub token_in: String,
    #[arg(short = 's', long = "spender")]
    pub spender: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
}
#[derive(Debug, Args)]
pub struct Swapethargs {
    #[arg(short = 'o', long = "outtoken")]
    pub token_out: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
    #[arg(short = 's', long = "slippage", default_value_t = 0.5)]
    pub slippage: f64,
}
#[derive(Debug, Args)]
pub struct Swaptokens {
    #[arg(short = 'i', long = "intoken")]
    pub token_in: String,
    #[arg(short = 'o', long = "outtoken")]
    pub token_out: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
    #[arg(short = 's', long = "slippage", default_value_t = 0.5)]
    pub slippage: f64,
}
#[derive(Debug, Args)]
pub struct Transfereth {
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
    #[arg(short = 'r', long = "recipient")]
    pub recipient: String,
}
#[derive(Debug, Args)]
pub struct Transfertoken {
    #[arg(short = 'i', long = "intoken")]
    pub token: String,
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
    #[arg(short = 'r', long = "recipient")]
    pub recipient: String,
}
#[derive(Debug, Args)]
pub struct Wrapargs {
    #[arg(short = 'a', long = "amount")]
    pub amount: String,
}
