# Rust CLI DeFi Bot (Ethereum/UniswapV2)

This command-line interface (CLI) tool empowers users to interact seamlessly with the Ethereum blockchain. Designed with simplicity and efficiency in mind, it supports various DeFi operations, including:

- Fetching on-chain data using your configured wallet, or other wallet
- Wrapping and unwrapping ETH with ease.
- Sending ETH securely to any recipient.
- Performing token swaps through UniswapV2 liquidity pools.

With this Rust-based bot, you can streamline your DeFi interactions while leveraging Ethereum's vast ecosystem.

## Setup .env

```
RPC=
PRIVATE_KEY=
```

## Run fork mainnet (wsl only!)

```shell
anvil --fork-url https://rpc.ankr.com/eth --steps-tracing --chain-id 31337
```

## Go to app

```
cd app
```

## Alias

### You can use token alias (config in alias_token.json):

- use "weth" instead of "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"

### You can use token alias (config in alias_address.json):

- use "deadAddress" instead of "0x000000000000000000000000000000000000dead"

## Get current wallet (For Wallet set on .env)

```shell
cargo run -- wallet
```

## Get ERC20 token metadata

(if in alias_address.json)

```shell
cargo run -- metadata -i 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2
cargo run -- metadata -i weth
```

## Get ETH balance (For Wallet set on .env)

```shell
cargo run -- balance
```

## Get ETH balance for an given address

```shell
cargo run -- balanceadd -r deadAddress
```

## Get balance of token ERC20 (For Wallet set on .env)

```shell
cargo run -- balanceof -i 0x6B175474E89094C44Da98b954EedeAC495271d0F
cargo run -- balanceof -i dai
```

## Get balance of token ERC20 for an given address

```shell
cargo run -- balanceofadd -i weth -r deadAddress
cargo run -- balanceofadd -i 0xdAC17F958D2ee523a2206206994597C13D831ec7 -r 0xdAC17F958D2ee523a2206206994597C13D831ec7
```

## Get allowance of token ERC20

```shell
cargo run -- allowance -i weth -r univ2Router
```

## Swap ETH into token ERC20

Swap 1 ETH for DAI with slippage at 1% (slippage by default 0.5%)

```shell
cargo run -- swapeth -o dai -a 1000000000000000000 -s 1
```

## Swap Tokens

Swap All DAI for WETH with slippage at 1% (slippage by default 0.5%)

```shell
cargo run -- swaptoken -i dai -o weth -a max -s 1
```

## Approve max an erc20 for a spender address (ex: approve DAImax for Univ2_Rouer)

```shell
cargo run -- approve -i dai -s univ2Router -a max
cargo run -- approve -i weth -s univ2Router -a max
```

## Transfer Token

```shell
cargo run -- transfertoken  -i weth -r deadAddress -a 1
cargo run -- transfereth -i weth -a 100000000 -r deadAddress
```

## Transfer ETH

```shell
cargo run -- transfereth -a 10000000000 -r deadAddress
```
