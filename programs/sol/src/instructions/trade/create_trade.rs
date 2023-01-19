use anchor_lang::prelude::*;
use anchor_spl::token::{
    self,
    {TokenAccount,Mint,Token},
    Transfer,
    SetAuthority,
    spl_token::instruction::AuthorityType
};
use anchor_lang::system_program;
use anchor_spl::metadata::MetadataAccount;
use crate::states::{Trade,TradeType,Collection};
use crate::{Errors, ID, MPL_TOKEN_METADATA_ID};

#[derive(Accounts)]
#[instruction(
    sol_amount: u64,
    spl_amount: u64
)]
pub struct CreateTrade<'info> {
    #[account(
        init,
        payer = party_one,
        space = Trade::LEN,
        seeds = [
            b"trade",
            party_one.key.as_ref(),
            party_two.key.as_ref(),
            collection_details.key().as_ref()
        ],
        bump
    )]
    pub trade_details: Account<'info, Trade>,

    #[account(
        init_if_needed,
        payer = party_one,
        seeds = [
            b"escrow-one",
            party_one.key.as_ref(),
            party_two.key.as_ref(),
            collection_details.key().as_ref()
        ],
        bump,
        token::mint = mint,
        token::authority = party_one
    )]
    pub escrow_party_one: Option<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = party_one,
        constraint = one_send_address.amount >= spl_amount @ Errors::InsufficientBalance
    )]
    pub one_send_address: Option<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = party_one.lamports() >= sol_amount @ Errors::InsufficientBalance
    )]
    pub party_one: Signer<'info>,

    pub mint: Option<Account<'info, Mint>>,

    #[account(
        token::authority = party_one,
        constraint = one_token_validation.amount == 1 @ Errors::TokenNotOne
    )]
    pub one_token_validation: Box<Account<'info,TokenAccount>>,

    #[account(
        seeds = [
            b"metadata", 
            MPL_TOKEN_METADATA_ID.as_ref(), 
            one_token_validation.mint.as_ref()
        ],
        seeds::program = MPL_TOKEN_METADATA_ID,
        bump,
        constraint = one_metadata_validation.collection.as_ref().unwrap().verified == 
        true @ Errors::CollectionNotVerified,
        constraint = one_metadata_validation.collection.as_ref().unwrap().key ==
        collection_details.verified_collection_key @ Errors::CollectionNotSame

    )]
    pub one_metadata_validation: Box<Account<'info,MetadataAccount>>,

    #[account(
        token::authority = party_two,
        constraint = two_token_validation.amount == 1 @ Errors::TokenNotOne
    )]
    pub two_token_validation: Box<Account<'info,TokenAccount>>,

    #[account(
        seeds = [
            b"metadata", 
            MPL_TOKEN_METADATA_ID.as_ref(), 
            two_token_validation.mint.as_ref()
        ],
        seeds::program = MPL_TOKEN_METADATA_ID,
        bump,
        constraint = two_metadata_validation.collection.as_ref().unwrap().verified == 
        true @ Errors::CollectionNotVerified,
        constraint = two_metadata_validation.collection.as_ref().unwrap().key ==
        collection_details.verified_collection_key @ Errors::CollectionNotSame
    )]
    pub two_metadata_validation: Box<Account<'info,MetadataAccount>>,

    /// CHECK: Nothing is read or written into this account
    pub party_two: AccountInfo<'info>,

    pub collection_details: Box<Account<'info, Collection>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> CreateTrade<'info> {
    pub fn transfer_spl(&self,spl_amount: u64) -> Result<()> {
        require_gt!(spl_amount,0, Errors::TokenAmountZero);
        
        match self.one_send_address.as_ref() {
            Some(token_account) => {
                let escrow = self.escrow_party_one.as_ref()
                .expect("Escrow account is not provided");

                token::transfer(
                    self.transfer_spl_context(token_account,escrow),
                    spl_amount
                )?;

                let (escrow_authority, _) = Pubkey::find_program_address(&[b"escrow"], &ID);

                token::set_authority(
                    self.set_authority_context(escrow), 
                    AuthorityType::AccountOwner, 
                    Some(escrow_authority)
                )?;
            },
            None => {
                return Err(Errors::AccountNotProvided.into());
            }
        }
        Ok(())
    }

    pub fn transfer_spl_context(
        &self, 
        one_send_address: &Account<'info, TokenAccount>,
        escrow: &Account<'info,TokenAccount>
    ) 
    -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: one_send_address.to_account_info(),
            to: escrow.to_account_info(),
            authority: self.party_one.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_sol_context(&self) -> CpiContext<'_,'_,'_,'info, system_program::Transfer<'info>> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = system_program::Transfer {
            from: self.party_one.to_account_info(),
            to: self.trade_details.to_account_info(),
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn set_authority_context(
        &self, 
        escrow: &Account<'info,TokenAccount>
    ) -> CpiContext<'_,'_,'_,'info, SetAuthority<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = SetAuthority {
            account_or_mint: escrow.to_account_info(),
            current_authority: self.party_one.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn create_trade_handler(
    ctx: Context<CreateTrade>,
    sol_amount: u64,
    spl_amount: u64,
    trade_type: TradeType
) -> Result<()> {
    let is_trade_active = ctx.accounts.collection_details.is_trade;
    require_eq!(is_trade_active,true, Errors::ModuleNotActive);

    let clock = Clock::get()?;
    let time = clock.unix_timestamp;

    match trade_type {
        TradeType::Sol => {
            require_gt!(sol_amount,0, Errors::TokenAmountZero);
            require_eq!(spl_amount,0, Errors::AmountNotZero);

            let escrow_party_one = &ctx.accounts.escrow_party_one;
            match escrow_party_one {
                Some(_escrow) => {
                    return Err(Errors::AccountNotRequired.into());
                },
                _ => ()
            }

            system_program::transfer(
                ctx.accounts.transfer_sol_context(),
                sol_amount
            )?;
        },
        TradeType::Spl => {            
            require_eq!(sol_amount,0, Errors::AmountNotZero);
            ctx.accounts.transfer_spl(spl_amount)?;
        },
        TradeType::Both => {
            require_gt!(sol_amount,0, Errors::TokenAmountZero);

            ctx.accounts.transfer_spl(spl_amount)?;

            system_program::transfer(
                ctx.accounts.transfer_sol_context(),
                sol_amount
            )?;
        }
    };

    let party_one = ctx.accounts.party_one.key();
    let party_two = ctx.accounts.party_two.key();
    let collection = ctx.accounts.collection_details.key();

    let one_send_address = if let Some(token_account) =
    ctx.accounts.one_send_address.as_ref() {
        Some(token_account.key())
    } else {
        None
    };

    let one_mint = if let Some(mint_account) =
    ctx.accounts.mint.as_ref() {
        if spl_amount > 0 {
            Some(mint_account.key())
        } else {
            None
        }
    } else {
        None
    };

    *ctx.accounts.trade_details = Trade::new(
        party_one, 
        party_two, 
        sol_amount, 
        spl_amount, 
        one_send_address, 
        time,
        collection,
        one_mint
    );

    Ok(())
}
