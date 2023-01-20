use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::MetadataAccount,
    token::{
        self,
        Mint,
        Token,
        SetAuthority,
        spl_token::instruction::AuthorityType
    }
};
use crate::{states::Collection,Errors,ID};

#[derive(Accounts)]
pub struct AddToken<'info> {
    #[account(
        mut,
        seeds = [b"collection", verified_collection.key().as_ref()],
        bump
    )]
    pub collection_details: Account<'info, Collection>,

    #[account(mut)]
    pub owner: Signer<'info>,
    
    #[account(
        constraint = verified_collection.collection == None @ Errors::NotCollectionNft,
        constraint = verified_collection.update_authority == owner.key()
        @ Errors::NotUpdateAuthority
    )]
    pub verified_collection: Account<'info, MetadataAccount>,

    #[account(
        init,
        payer = owner,
        mint::decimals = 9,
        mint::authority = owner,
    )]
    pub token: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> AddToken<'info> {
    pub fn set_authority_context(&self) -> CpiContext<'_,'_,'_,'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: self.token.to_account_info(),
            current_authority: self.owner.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn add_token_handler(ctx: Context<AddToken>) -> Result<()> {
    let token_mint = ctx.accounts.collection_details.token_mint;

    match token_mint {
        Some(_key) => {
            return Err(Errors::TokenAlreadyExists.into());
        },
        None => {
            let (token_authority, _) = Pubkey::find_program_address(&[b"token-authority"], &ID);

            token::set_authority(
                ctx.accounts.set_authority_context(), 
                AuthorityType::MintTokens, 
                Some(token_authority)
            )?;

            let collection_details = &mut ctx.accounts.collection_details;

            collection_details.token_mint = Some(ctx.accounts.token.key());  
        }
    }

    Ok(())
}
