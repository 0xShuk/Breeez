// use anchor_lang::prelude::*;
// use anchor_spl::metadata::MetadataAccount;
// use crate::states::Collection;

// #[derive(Accounts)]
// pub struct AddCommunity<'info> {
//     #[account(
//         mut,
//         seeds = [b"collection", verified_collection.key().as_ref()],
//         bump
//     )]
//     pub collection_details: Account<'info, Collection>,

//     #[account(mut)]
//     pub owner: Signer<'info>,
    
//     #[account(
//         constraint = verified_collection.collection == None,
//         constraint = verified_collection.update_authority == owner.key()
//     )]
//     pub verified_collection: Account<'info, MetadataAccount>,

// }

// pub fn add_community(ctx: Context<AddCommunity>) -> Result<()> {
    
//     Ok(())
// }
