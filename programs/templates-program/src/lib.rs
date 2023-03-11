use anchor_lang::prelude::*;

declare_id!("DDr6GWEUMARbpwThziuFpwCUkmWAG1htNyQpfdwZJf2G"); 

// Program entrypoint - this is the main function that is called when the program is invoked
#[program]
pub mod template_program { //🐍 case
    use super::*;
   
    pub fn create_template(ctx: Context<CreateTemplateCxt>, arweave_id: String, original: Option<Pubkey>, archived: bool) -> Result<()> { //contexts + sides of fries 🐍 LABEL
    // extract from context and write
    let payload: &mut Account<TemplateAccount> = &mut ctx.accounts.template; 
    let owner: &Signer = &ctx.accounts.owner; 
    
    if arweave_id.chars().count() != 64 {
        return Err(ErrorCode::IdIncorrectLength.into())
    }

    payload.owner = *owner.key; // * is dereferencing - get value of pointer
    payload.arweave_id = arweave_id;
    payload.original = original;
    payload.archived = archived;
    payload.version = 0;

    Ok(()) //handle error
}

pub fn update_template(ctx: Context<UpdateTemplateCxt>, archived: bool, _version: u8) -> Result<()> { //contexts + sides of fries 🐍 LABEL
    // extract from context and write
    let payload: &mut Account<TemplateAccount> = &mut ctx.accounts.template; 
    
    if payload.version + 1 != _version {
        return Err(ErrorCode::InvalidVersion.into())
    }

    payload.archived = archived;
    payload.version = _version;
    Ok(()) //handle error
}


}

// Account context - Feed all accounts that will be interacted with
#[derive(Accounts)]
pub struct CreateTemplateCxt<'info> { //'info is rust lifetime - tells compiler that this account will be used for the duration of the program
    #[account(init, payer = payer, space = TemplateAccount::LEN)]
    pub template: Account<'info, TemplateAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, 
    pub system_program: Program<'info, System>, //Program is part of prelude - ensure we're using actual system acct // TODO - is this unnecessary?
}

#[derive(Accounts)]
pub struct UpdateTemplateCxt<'info> { 
    #[account(mut, has_one = owner)] //anchor will reject if owner mismatch & notice mut on account
    pub template: Account<'info, TemplateAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, //mut?
}

// Account definition - more info at: https://lorisleiva.com/create-a-solana-dapp-from-scratch/structuring-our-tweet-account
#[account]
pub struct TemplateAccount {
    pub owner: Pubkey,
    pub arweave_id: String,
    pub original: Option<Pubkey>,
    pub archived: bool,
    pub version: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8; //part of all of new accounts - stores type of account
const PUBLIC_KEY_LENGTH: usize = 32; //author
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const ID_LENGTH: usize = 64 * 4; // SHA256 * 4 bytes pser char
const ARCHIVED_LENGTH: usize = 1; // Boolean
const VERSION_LENGTH: usize = 1; // u8

impl TemplateAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner pubkey
        + STRING_LENGTH_PREFIX + ID_LENGTH // Arweave ID - string
        + PUBLIC_KEY_LENGTH // original pubkey
        + ARCHIVED_LENGTH
        + VERSION_LENGTH;   
}

#[error_code]
pub enum ErrorCode {
    #[msg("arweave_id is not the correct length.")]
    IdIncorrectLength,
    #[msg("this is an invalid version.")]
    InvalidVersion,

}