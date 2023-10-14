use anchor_lang::prelude::*;
use anchor_lang::solana_program::{entrypoint::ProgramResult, program::invoke_signed};
use anchor_spl::{associated_token, token};
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::error::ErrorCode;
use crate::state::LoanEscrow;
use crate::util::{to_pubkey, transfer_sol, transfer_token_signed, SOL_USD_PRICE_FEED_ID};

pub fn accept_loan(ctx: Context<AcceptLoan>, loan_id: u8) -> Result<()> {
    msg!("Get SOL/USDC price conversion data");
    if !ctx
        .accounts
        .pyth_account
        .key()
        .eq(&to_pubkey(SOL_USD_PRICE_FEED_ID))
    {
        return Err(ErrorCode::PythError.into());
    }
    let sol_usd_price_feed = load_price_feed_from_account_info(&ctx.accounts.pyth_account).unwrap();
    // TODO
    let _current_price = sol_usd_price_feed.get_price_unchecked();

    msg!("Calculate the required collateral");
    let amount_in_lamports = 100000;

    msg!("Take the borrower's SOL as collateral");
    transfer_sol(
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.borrower.to_account_info(),
        ctx.accounts.loan_escrow.to_account_info(),
        amount_in_lamports,
    )?;

    msg!("Mint receipt tokens to the lender");
    mint_receipt_tokens_to_lender(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.loan_note_mint.to_account_info(),
        ctx.accounts.lender_loan_note_mint_ata.to_account_info(),
        ctx.accounts.loan_escrow.to_account_info(),
        loan_id,
        ctx.accounts.loan_escrow.bump,
        ctx.accounts.loan_escrow.deposit,
    )?;

    msg!("Transfer the USDC loan out to the borrower");
    transfer_token_signed(
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.loan_escrow.to_account_info(),
        ctx.accounts.loan_escrow_usdc_ata.to_account_info(),
        ctx.accounts.borrower_usdc_ata.to_account_info(),
        loan_id,
        ctx.accounts.loan_escrow.bump,
        ctx.accounts.loan_escrow_usdc_ata.amount,
    )?;

    ctx.accounts.loan_escrow.borrower = ctx.accounts.borrower.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(loan_id: u8)]
pub struct AcceptLoan<'info> {
    pub usdc_mint: Box<Account<'info, token::Mint>>,

    #[account(
        init,
        payer = borrower,
        seeds = [
            b"loan_note_mint",
            loan_escrow.key().as_ref(),
        ],
        bump,
        mint::decimals = 6,
        mint::authority = loan_escrow,
    )]
    pub loan_note_mint: Box<Account<'info, token::Mint>>,

    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = loan_note_mint,
        associated_token::authority = lender,
    )]
    pub lender_loan_note_mint_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        seeds = [
            LoanEscrow::SEED_PREFIX.as_bytes().as_ref(),
            loan_id.to_le_bytes().as_ref(),
        ],
        bump = loan_escrow.bump,
    )]
    pub loan_escrow: Box<Account<'info, LoanEscrow>>,
    #[account(
        init_if_needed,
        payer = borrower,
        associated_token::mint = usdc_mint,
        associated_token::authority = loan_escrow,
    )]
    pub loan_escrow_usdc_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(mut)]
    pub borrower: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = borrower,
    )]
    pub borrower_usdc_ata: Box<Account<'info, token::TokenAccount>>,

    #[account(
        mut,
        address = loan_escrow.lender,
    )]
    pub lender: SystemAccount<'info>,

    pub pyth_account: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub associated_token_program: Program<'info, associated_token::AssociatedToken>,
}

fn mint_receipt_tokens_to_lender<'a>(
    token_program: AccountInfo<'a>,
    loan_note_mint: AccountInfo<'a>,
    lender_loan_note_mint_ata: AccountInfo<'a>,
    loan_escrow: AccountInfo<'a>,
    loan_id: u8,
    loan_escrow_bump: u8,
    deposit: u64,
) -> Result<()> {
    token::mint_to(
        CpiContext::new_with_signer(
            token_program,
            token::MintTo {
                mint: loan_note_mint,
                to: lender_loan_note_mint_ata,
                authority: loan_escrow,
            },
            &[&[
                LoanEscrow::SEED_PREFIX.as_bytes().as_ref(),
                loan_id.to_le_bytes().as_ref(),
                &[loan_escrow_bump],
            ]],
        ),
        deposit,
    )
}
