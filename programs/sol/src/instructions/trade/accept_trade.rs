use anchor_lang::prelude::*;
use anchor_spl::{token::{
    self,
    {TokenAccount,Mint,Token},
    Transfer,
    SetAuthority,
    spl_token::instruction::AuthorityType
}, associated_token::AssociatedToken};
use anchor_lang::system_program;
use crate::states::{Trade,TradeType, Collection};
use crate::{Errors, ID,};

#[derive(Accounts)]
#[instruction(
    sol_amount: u64,
    spl_amount: u64
)]
pub struct AcceptTrade<'info> {
    #[account(
        mut,
        has_one = party_two,
        has_one = collection
    )]
    pub trade_details: Box<Account<'info, Trade>>,

    #[account(
        init_if_needed,
        payer = party_two,
        seeds = [
            b"escrow-two",
            trade_details.party_one.as_ref(),
            party_two.key.as_ref(),
            collection.key().as_ref()
        ],
        bump,
        token::mint = two_mint,
        token::authority = party_two
    )]
    pub escrow_party_two: Option<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = party_two, 
        associated_token::mint = one_mint, 
        associated_token::authority = party_two
    )]
    pub two_receive_address: Option<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = two_mint,
        token::authority = party_two,
        constraint = two_send_address.amount >= spl_amount @ Errors::InsufficientBalance
    )]
    pub two_send_address: Option<Account<'info, TokenAccount>>,

    #[account(
        constraint = one_mint.key() == trade_details.one_mint.unwrap() @ Errors::MintNotExist
    )]
    pub one_mint: Option<Account<'info,Mint>>,

    pub two_mint: Option<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = party_two.lamports() >= sol_amount @ Errors::InsufficientBalance
    )]
    pub party_two: Signer<'info>,
    
    pub collection: Box<Account<'info, Collection>>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> AcceptTrade<'info> {
    pub fn transfer_spl(&self,spl_amount: u64) -> Result<()> {
        require_gt!(spl_amount,0, Errors::TokenAmountZero);

        match self.two_send_address.as_ref() {
            Some(token_account) => {
                let escrow = self.escrow_party_two.as_ref()
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
        two_send_address: &Account<'info, TokenAccount>,
        escrow: &Account<'info,TokenAccount>
    ) 
    -> CpiContext<'_,'_,'_,'info, Transfer<'info>> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: two_send_address.to_account_info(),
            to: escrow.to_account_info(),
            authority: self.party_two.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }

    pub fn transfer_sol_context(&self) -> CpiContext<'_,'_,'_,'info, system_program::Transfer<'info>> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = system_program::Transfer {
            from: self.party_two.to_account_info(),
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
            current_authority: self.party_two.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}

pub fn accept_trade_handler(
    ctx: Context<AcceptTrade>,
    sol_amount: u64,
    spl_amount: u64,
    trade_type: TradeType
) -> Result<()> {
    let confirm_status = ctx.accounts.trade_details.is_confirmed;
    
    require_eq!(confirm_status,false,Errors::TradeAlreadyAccepted);

    match trade_type {
        TradeType::Sol => {
            require_gt!(sol_amount,0, Errors::TokenAmountZero);
            require_eq!(spl_amount,0, Errors::AmountNotZero);

            let escrow_party_two = &ctx.accounts.escrow_party_two;
            match escrow_party_two {
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

    let two_send_address = if let Some(token_account) =
    ctx.accounts.two_send_address.as_ref() {
        Some(token_account.key())
    } else {
        None
    };

    let two_mint = if let Some(mint_account) =
    ctx.accounts.two_mint.as_ref() {
        if spl_amount > 0 {
            Some(mint_account.key())
        } else {
            None
        }
    } else {
        None
    };

    let two_receive_address = if let Some(token_account) =
    ctx.accounts.two_receive_address.as_ref() {
        Some(token_account.key())
    } else {
        None
    };

    let one_mint_wrap = ctx.accounts.trade_details.one_mint;

    if let Some(_mint) = one_mint_wrap {
        match two_receive_address {
            None => {
                return Err(Errors::AccountNotProvided.into());
            },
            _ => ()
        }
    }

    let trade_details = &mut ctx.accounts.trade_details;

    trade_details.is_confirmed = true;
    trade_details.two_receive_address = two_receive_address;
    trade_details.two_send_address = two_send_address;
    trade_details.two_mint = two_mint;
    trade_details.sol_amount[1] = sol_amount;
    trade_details.spl_amount[1] = spl_amount;
    
    Ok(())
}
