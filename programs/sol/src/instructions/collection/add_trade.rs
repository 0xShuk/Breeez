use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::{states::Collection,Errors};

#[derive(Accounts)]
pub struct AddTrade<'info> {
    #[account(
        mut,
        seeds = [b"collection", verified_collection.key().as_ref()],
        bump
    )]
    pub collection_details: Account<'info, Collection>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        constraint = verified_collection.collection == None,
        constraint = verified_collection.update_authority == owner.key()
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

}

pub fn add_trade_handler(ctx: Context<AddTrade>, trade_fee: u64, duration: i64) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    require_eq!(collection_details.is_trade, false, Errors::ModuleAlreadyAdded);
    
    collection_details.is_trade = true;
    collection_details.trade_duration = duration;
    collection_details.trade_fees = trade_fee;
    Ok(())
}
