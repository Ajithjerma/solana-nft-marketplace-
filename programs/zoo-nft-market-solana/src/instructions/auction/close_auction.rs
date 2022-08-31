use anchor_lang::prelude::*;
use anchor_lang::{AccountDeserialize, AnchorDeserialize};
use anchor_spl::token::{self, TokenAccount, Transfer};
use anchor_lang::accounts::program_account::ProgramAccount;
 use anchor_lang::accounts::cpi_account::CpiAccount;
use crate::instructions::auction::bid::Auction;
use anchor_lang::account;

pub fn close_auction(ctx: Context<CloseAuction>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;

        let (_, seed) =
            Pubkey::find_program_address(&[&auction.seller.to_bytes()], &ctx.program_id);
        let seeds = &[auction.seller.as_ref(), &[seed]];
        let signer = &[&seeds[..]];

        // item ownership transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.item_holder.to_account_info().clone(),
            to: ctx.accounts.item_receiver.to_account_info().clone(),
            authority: ctx.accounts.item_holder_auth.clone(),
        };
        let cpi_program = ctx.accounts.token_program.clone();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, ctx.accounts.item_holder.amount)?;

        // currency ownership transfer
        if ctx.accounts.currency_holder.amount >= auction.price {
            let cpi_accounts = Transfer {
                from: ctx.accounts.currency_holder.to_account_info().clone(),
                to: ctx.accounts.currency_receiver.to_account_info().clone(),
                authority: ctx.accounts.currency_holder_auth.clone(),
            };
            let cpi_program = ctx.accounts.token_program.clone();
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
            token::transfer(cpi_ctx, auction.price)?;
        }

        auction.ongoing = false;
        Ok(())
    }
    
    
    
    #[derive(Accounts)]
pub struct CloseAuction<'info> {
    #[account(mut, "auction.ongoing")]
    auction: ProgramAccount<'info, Auction>,
    #[account(signer)]
    seller: AccountInfo<'info>,
    #[account(
        mut,
        "item_holder.to_account_info().key == &auction.item_holder",
        "&item_holder.owner == &Pubkey::find_program_address(&[&seller.key.to_bytes()], &program_id).0"
    )]
    item_holder: CpiAccount<'info, TokenAccount>,
    item_holder_auth: AccountInfo<'info>,
    #[account(mut, "item_receiver.owner == auction.bidder")]
    item_receiver: CpiAccount<'info, TokenAccount>,
    #[account(
        mut,
        "currency_holder.to_account_info().key == &auction.currency_holder",
        "&currency_holder.owner == &Pubkey::find_program_address(&[&seller.key.to_bytes()], &program_id).0"
    )]
    currency_holder: CpiAccount<'info, TokenAccount>,
    #[account("&currency_holder.owner == currency_holder_auth.key")]
    currency_holder_auth: AccountInfo<'info>,
    #[account(mut)]
    currency_receiver: CpiAccount<'info, TokenAccount>,
    #[account("token_program.key == &token::ID")]
    token_program: AccountInfo<'info>,
}
