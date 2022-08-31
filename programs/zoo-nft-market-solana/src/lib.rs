mod instructions;
mod states;
mod error;
use borsh::{BorshSerialize, BorshDeserialize};

use anchor_lang::prelude::*;
use instructions::mint::mint::*;
use instructions::market::create_order::*;
use instructions::market::cancel_order::*;
use instructions::market::fill_order::*;
use instructions::auction::create_auction::*;
use instructions::auction::bid::*;
use instructions::auction::close_auction::*;
use crate::instructions::auction::create_auction::CreateAuction;
use crate::instructions::auction::bid::Bid;
use crate::instructions::auction::close_auction::CloseAuction;
declare_id!("D6oUwPksdxCJLdiJwUUCn6XPGsUXAsXhPdsMfiULPkLa");

#[program]
pub mod zoo_nft_market_solana {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        instructions::mint::mint::mint_nft(ctx, creator_key, name, symbol, uri)
    }

    pub fn create_order(
        ctx: Context<CreateOrder>,
        memo: String,
        price: u64
    ) -> Result<()> {
        instructions::market::create_order::create_order(ctx, memo, price)
    }

    pub fn cancel_order(
        ctx: Context<CancelOrder>
    ) -> Result<()> {
        instructions::market::cancel_order::cancel_order(ctx)
    }

    pub fn fill_order(ctx: Context<FillOrder>) -> Result<()> {
        instructions::market::fill_order::fill_order(ctx)
    }
    
    pub fn create_auction(
    ctx: Context<CreateAuction>, start_price: u64) 
    -> Result<()> {
    instructions::auction::create_auction::create_auction(ctx,start_price)
    }
    pub fn bid(ctx: Context<Bid>, price: u64) -> Result<()> {
      instructions::auction::bid::bid(ctx,price)
      }
      
    pub fn close_auction(ctx: Context<CloseAuction>) -> Result<()> {
      instructions::auction::close_auction::close_auction(ctx)
      }
}
