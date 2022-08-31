use anchor_lang::prelude::*;
use anchor_lang::{AccountDeserialize, AnchorDeserialize};
use anchor_spl::token::{self, TokenAccount, Transfer};
use anchor_lang::accounts::program_account::ProgramAccount;
 use anchor_lang::accounts::cpi_account::CpiAccount;
use anchor_lang::account;


    pub fn create_auction(ctx: Context<CreateAuction>, start_price: u64) -> Result<()> {
        // init auction
        let auction = &mut ctx.accounts.auction;
        auction.ongoing = true;
        auction.seller = *ctx.accounts.seller.key;
        auction.item_holder = *ctx.accounts.item_holder.to_account_info().key;
        auction.currency_holder = *ctx.accounts.currency_holder.to_account_info().key;
        auction.bidder = *ctx.accounts.seller.key;
        auction.price = start_price;
        Ok(())
    }
    

    
#[derive(Accounts)]
pub struct CreateAuction<'info> {
    #[account(init)]
    auction: ProgramAccount<'info, Auction>,
    seller: AccountInfo<'info>,
    #[account("&item_holder.owner == &Pubkey::find_program_address(&[&seller.key.to_bytes()], &program_id).0")]
    item_holder: CpiAccount<'info, TokenAccount>,
    #[account("&currency_holder.owner == &Pubkey::find_program_address(&[&seller.key.to_bytes()], &program_id).0")]
    currency_holder: CpiAccount<'info, TokenAccount>,
    rent: Sysvar<'info, Rent>,
}



#[account]
pub struct Auction {
    ongoing: bool,
    seller: Pubkey,
    item_holder: Pubkey,
    currency_holder: Pubkey,
    bidder: Pubkey,
    refund_receiver: Pubkey,
    price: u64,
}  

