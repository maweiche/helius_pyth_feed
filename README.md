# Pyth USDC -> SOL Program

This repo contains the rust program and test code that allows you to swap USDC for SOL on the Solana blockchain (devnet) using the Pyth oracle. User can input the amount they want to send in USDC and the program will calculate the live exchange rate and send the appropriate amount of SOL to the user's wallet.


## How to run

1. Clone the repo
2. Run `anchor build` to build the program
3. Run `anchor deploy` to deploy the program to the devnet
4. Copy the deployed program id from the terminal to the `Anchor.toml` file and `declare_id!()` line in the `lib.rs`
5. Run `anchor build` & `anchor deploy` again
6. Run `anchor test` to run the test code

PYTH Devnet USDC/SOL price feed: "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"