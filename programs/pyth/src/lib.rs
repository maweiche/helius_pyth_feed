use anchor_lang::prelude::*;
use pyth_sdk_solana::*;

declare_id!("58V9VYXJ5QxWaFPNC8bq6xSnQJsWrDW39fjSkQnrx9Jn");
// This is the price feed id for SOL/USD on devnet
// https://pyth.network/developers/price-feed-ids#solana-devnet
const SOL_USD_PRICEFEED_ID : &str = "J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix";
const STALENESS_THRESHOLD : u64 = 60; // staleness threshold in seconds

#[program]
pub mod pyth {
    use std::str::FromStr;
    use anchor_lang::solana_program::{system_instruction, native_token::LAMPORTS_PER_SOL, program::invoke};
    use super::*;

    pub fn pay_usd(ctx: Context<PayUSD>, amount : u64) -> Result<()> {
        
        if Pubkey::from_str(SOL_USD_PRICEFEED_ID) != Ok(ctx.accounts.sol_usd_price_account.key()){
            return Err(error!(CustomError::WrongPriceFeedId))
        };
        let sol_usd_price_feed = load_price_feed_from_account_info(&ctx.accounts.sol_usd_price_account).unwrap();
        let current_timestamp = Clock::get()?.unix_timestamp;
        let current_price: Price = sol_usd_price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();
        let amount_in_lamports = amount *  LAMPORTS_PER_SOL * 10u64.pow(u32::try_from(-current_price.expo).unwrap()) / (u64::try_from(current_price.price).unwrap());
        let transfer_instruction = system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.to.key(), amount_in_lamports);
        invoke(&transfer_instruction, &ctx.accounts.to_account_infos())?;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(amount : u64)]
pub struct PayUSD<'info> {
    pub from : Signer<'info>,
    /// CHECK : This is an unchecked receiver account
    #[account(mut)]
    pub to : AccountInfo<'info>,
    /// CHECK : We will manually check this against the Pubkey of the price feed
    pub sol_usd_price_account : AccountInfo<'info>,
    pub system_program : Program<'info, System>
}

#[error_code]
pub enum CustomError {
    WrongPriceFeedId
}