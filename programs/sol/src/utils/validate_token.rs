use anchor_lang::prelude::*;
use anchor_spl::token::{
    TokenAccount
};
use crate::Errors;

pub fn validate_token_account(
    token_accountinfo: &AccountInfo,
    signer: &Pubkey,
) -> Result<Pubkey> {

    // Verify whether the token account is initialized
    require_eq!(token_accountinfo.data_is_empty(), false, Errors::AccountNotInitialized);

    // Try to derive TokenAccount from the given AccountInfo
    let token_account: Account<TokenAccount> = Account::try_from(token_accountinfo)?;

    // Asserts whether the signer has the authority of the token account
    assert_eq!(token_account.owner, *signer);

    // Verifies whether the supply of the token account is exactly 1
    require_eq!(token_account.amount, 1, Errors::TokenNotOne);

    let mint = token_account.mint;

    Ok(mint)
}