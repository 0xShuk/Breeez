use anchor_lang::prelude::*;
use anchor_spl::{
    token::{TokenAccount,Mint,Token,Transfer,SetAuthority,
        spl_token::instruction::AuthorityType, self
    },
    metadata::MetadataAccount
};

use crate::states::{Collection,Stake};
use crate::{Errors, MPL_TOKEN_METADATA_ID, ID};

#[derive(Accounts)]
pub struct StakeNft<'info> {
    #[account(
        init,
        payer = signer,
        space = Stake::LEN,
        seeds = [
            b"stake",
            mint.key().as_ref()
        ],
        bump
    )]
    pub stake_details: Account<'info,Stake>,

    #[account(
        init,
        payer = signer,
        seeds = [
            b"nft-escrow",
            mint.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = signer
    )]
    pub nft_escrow: Account<'info,TokenAccount>,

    pub collection_details: Box<Account<'info,Collection>>,

    #[account(
        mut,
        token::authority = signer, 
        token::mint = mint, 
        constraint = nft_send_address.amount == 1 @ Errors::TokenNotOne
    )]
    pub nft_send_address: Account<'info,TokenAccount>,

    #[account(mint::decimals = 0)]
    pub mint: Account<'info, Mint>,
    
    #[account(
        seeds = [
            b"metadata", 
            MPL_TOKEN_METADATA_ID.as_ref(), 
            mint.key().as_ref()
        ],
        seeds::program = MPL_TOKEN_METADATA_ID,
        bump,
        constraint = metadata.collection.as_ref().unwrap().verified @ Errors::CollectionNotVerified,
        constraint = metadata.collection.as_ref().unwrap().key ==
        collection_details.verified_collection_key @ Errors::CollectionNotSame

    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>
}

impl<'info> StakeNft<'info> {
    pub fn transfer_nft_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.nft_send_address.to_account_info(),
            to: self.nft_escrow.to_account_info(),
            authority: self.signer.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn set_authority_context(&self) -> CpiContext<'_,'_,'_,'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: self.nft_escrow.to_account_info(),
            current_authority: self.signer.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn stake_nft_handler(ctx: Context<StakeNft>) -> Result<()> {
    let collection_details = &ctx.accounts.collection_details;
    let nft_send_address = ctx.accounts.nft_send_address.key();
    let staker = ctx.accounts.signer.key();

    let is_stake_active = collection_details.is_staking;
    require_eq!(is_stake_active,true, Errors::ModuleNotActive);

    token::transfer(ctx.accounts.transfer_nft_context(), 1)?;

    let (nft_authority, _) = Pubkey::find_program_address(&[b"nft-authority"], &ID);

    token::set_authority(
        ctx.accounts.set_authority_context(), 
        AuthorityType::AccountOwner, 
        Some(nft_authority)
    )?;

    let stake_details = &mut ctx.accounts.stake_details;

    **stake_details = Stake::new(
        staker,
        nft_send_address,
        collection_details.key()
    );

    Ok(())
}
