use anchor_lang::prelude::*;
use anchor_spl::{
    token::{
        TokenAccount, Mint,
        Token, MintTo,
        self
    },
    associated_token::AssociatedToken
};

use crate::states::{Collection,Stake};
use crate::{ ID, utils};

#[derive(Accounts)]
pub struct WithdrawTokens<'info> {
    #[account(
        mut,
        seeds = [
            b"stake",
            nft_mint.key().as_ref()
        ],
        bump,
        has_one = collection,
        has_one = owner,
    )]
    pub stake_details: Box<Account<'info,Stake>>,

    pub collection: Box<Account<'info,Collection>>,

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

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info,System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> WithdrawTokens<'info> {
    pub fn mint_token_context(&self) -> CpiContext<'_,'_,'_,'info, MintTo<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.token_mint.to_account_info(),
            to: self.token_receive_address.to_account_info(),
            authority: self.token_authority.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn withdraw_tokens_handler(ctx: Context<WithdrawTokens>) -> Result<()> {
    let emission = ctx.accounts.collection.emission;
    let stake_time = ctx.accounts.stake_details.time;

    let reward = utils::calc_emission(stake_time, emission);

    let (_token_authority, token_bump) = Pubkey::find_program_address(&[b"token-authority"], &ID);
    let token_seed = &[&b"token-authority"[..], &[token_bump]];

    token::mint_to(ctx.accounts.mint_token_context().with_signer(&[&token_seed[..]]), reward)?;

    let clock = Clock::get().unwrap();
    let current_time = clock.unix_timestamp;

    ctx.accounts.stake_details.time = current_time;
    Ok(())
}

