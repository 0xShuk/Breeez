use anchor_lang::prelude::*;
use anchor_spl::metadata::MetadataAccount;
use crate::states::Collection;

#[derive(Accounts)]
pub struct EditTrade<'info> {
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

pub fn edit_trade_handler(ctx: Context<EditTrade>, new_num: u64, edit_type: TradeEditType) -> Result<()> {
    let collection_details = &mut ctx.accounts.collection_details;

    match edit_type {
        TradeEditType::Fee => {
            collection_details.trade_fees = new_num;
        },
        TradeEditType::Duration => {
            collection_details.trade_duration = new_num as i64;
        }
    }
    Ok(())
}

#[derive(AnchorSerialize,AnchorDeserialize)]
pub enum TradeEditType {
    Fee,
    Duration,
}