#![allow(clippy::result_large_err)]
// Import the necessary modules
use anchor_lang::prelude::*;
use anchor_spl::token_interface::*;
// Declare the program's ID
declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");
// Define the program
#[program]
pub mod tokenvesting {
    use super::*;
    // Define the create_vesting_account function
    pub fn create_vesting_account(
        ctx: Context<CreateVestingAccount>,
        company_name: String,
    ) -> Result<()> {
        *ctx.accounts.vesting_account = VestingAccount {
            owner: ctx.accounts.signer.key(),
            mint: ctx.accounts.mint.key(),
            treasury_token_account: ctx.accounts.treasury_token_account.key(),
            company_name,
            treasury_bump: ctx.bumps.treasury_token_account,
            bump: ctx.bumps.vesting_account,
        };

        Ok(())
    }
}
// Create the CreateVestingAccount struct, which will be used to create a new vesting account
// The struct has two fields: signer and vesting_account
// The signer field is a Signer object, which is used to sign the transaction
// The vesting_account field is an Account object, which is used to store the vesting account data
// The struct is annotated with the Accounts attribute, which specifies the accounts that will be used in the function
#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // The vesting_account field is annotated with the init attribute, which specifies that the account will be initialized
    // The payer field specifies the account that will pay for the transaction
    // The space field specifies the size of the account data
    // The seeds field specifies the seeds that will be used to derive the account's address
    // The bump field specifies the bump seed that will be used to derive the account's address
    #[account(
        init,
        payer = signer,
        space = 8 + VestingAccount::INIT_SPACE,
        seeds = [company_name.as_ref()],
        bump,
    )]
    pub vesting_account: Account<'info, VestingAccount>,
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(init,
        token::mint = mint,
        token::authority = treasury_token_account,
        payer = signer,
        seeds = [b"vesting", company_name.as_bytes()],
        bump
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
// Define the VestingAccount struct, which will be used to store the vesting account data
// The struct has six fields: owner, mint, treasury_token_account, company_name, treasury_bump, and bump
// The owner field is a Pubkey object, which specifies the owner of the vesting account
// The mint field is a Pubkey object, which specifies the mint of the vesting account
// The treasury_token_account field is a Pubkey object, which specifies the treasury token account of the vesting account
// The company_name field is a String object, which specifies the company name of the vesting account
// The treasury_bump field is a u8 object, which specifies the treasury bump of the vesting account
// The bump field is a u8 object, which specifies the bump of the vesting account
// The struct is annotated with the Account attribute, which specifies that the struct will be used as an account
#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub treasury_token_account: Pubkey,

    #[max_len(50)]
    pub company_name: String,

    pub treasury_bump: u8,
    pub bump: u8,
}
