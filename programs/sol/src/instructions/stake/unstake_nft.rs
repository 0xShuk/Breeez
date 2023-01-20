use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        TokenAccount,Mint,Token,Transfer,MintTo,
        self, CloseAccount
    },
    associated_token::AssociatedToken
};

use crate::states::{Collection,Stake};
use crate::{ ID, utils};

#[derive(Accounts)]
pub struct UnstakeNft<'info> {
    #[account(
        mut,
        seeds = [
            b"stake",
            nft_mint.key().as_ref()
        ],
        bump,
        has_one = collection,
        has_one = nft_send_address,
        has_one = owner,
        close = owner
    )]
    pub stake_details: Box<Account<'info,Stake>>,

    #[account(
        mut,
        seeds = [
            b"nft-escrow",
            nft_mint.key().as_ref()
        ],
        bump,
        token::mint = nft_mint,
    )]
    pub nft_escrow: Account<'info,TokenAccount>,

    pub collection: Box<Account<'info,Collection>>,

    #[account(
        init_if_needed,
        payer = owner, 
        associated_token::mint = nft_mint, 
        associated_token::authority = owner
    )]
    pub nft_send_address: Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer = owner, 
        associated_token::mint = token_mint, 
        associated_token::authority = owner
    )]
    pub token_receive_address: Account<'info,TokenAccount>,

    #[account(mint::decimals = 0)]
    pub nft_mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"token-authority"],
        bump
    )]
    pub token_authority: AccountInfo<'info>,

    #[account(
        seeds = [b"nft-authority"],
        bump
    )]
    pub nft_authority: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> UnstakeNft<'info> {
    pub fn transfer_nft_context(&self) -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.nft_escrow.to_account_info(),
            to: self.nft_send_address.to_account_info(),
            authority: self.nft_authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn mint_token_context(&self) -> CpiContext<'_,'_,'_,'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.token_receive_address.to_account_info(),
            authority: self.token_authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn close_account_context(&self) -> CpiContext<'_,'_,'_,'info, CloseAccount<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = CloseAccount {
            account: self.nft_escrow.to_account_info(),
            destination: self.owner.to_account_info(),
            authority: self.nft_authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn unstake_nft_handler(ctx: Context<UnstakeNft>) -> Result<()> {
    let emission = ctx.accounts.collection.emission;
    let stake_time = ctx.accounts.stake_details.time;

    let reward = utils::calc_emission(stake_time, emission);

    let (_nft_authority, nft_bump) = Pubkey::find_program_address(&[b"nft-authority"], &ID);
    let nft_seed = &[&b"nft-authority"[..], &[nft_bump]];

    let (_token_authority, token_bump) = Pubkey::find_program_address(&[b"token-authority"], &ID);
    let token_seed = &[&b"token-authority"[..], &[token_bump]];

    token::transfer(ctx.accounts.transfer_nft_context().with_signer(&[&nft_seed[..]]), 1)?;

    token::mint_to(ctx.accounts.mint_token_context().with_signer(&[&token_seed[..]]), reward)?;

    token::close_account(ctx.accounts.close_account_context().with_signer(&[&nft_seed[..]]))?;

    Ok(())
}

