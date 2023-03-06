use anchor_lang::prelude::*;

declare_id!("2rasBfCUQYPAcn2AjCYFvzjSokRYbrZJV3oeu2W696nB");

// Program entrypoint - this is the main function that is called when the program is invoked
#[program]
pub mod taxonomy_program { //🐍 case
    use super::*;
   
    pub fn create_taxonomy(ctx: Context<CreateTaxonomyCxt>, label: String, parent: Option<Pubkey>) -> Result<()> { //contexts + sides of fries 🐍 LABEL
    // extract from context and write
    let payload: &mut Account<TaxonomyAccount> = &mut ctx.accounts.taxonomy; 
    let owner: &Signer = &ctx.accounts.owner; 
    
    if label.chars().count() > 24 {
        return Err(ErrorCode::LabelTooLong.into())
    }

    if label.chars().count() < 1 {
        return Err(ErrorCode::LabelTooShort.into())
    }

    payload.owner = *owner.key; // * is dereferencing - get value of pointer
    payload.label = label;
    payload.parent = parent; //?

    Ok(()) //handle error
}

pub fn update_taxonomy(ctx: Context<UpdateTaxonomyCxt>, label: String, parent: Option<Pubkey>) -> Result<()> { //contexts + sides of fries 🐍 LABEL
    // extract from context and write
    let payload: &mut Account<TaxonomyAccount> = &mut ctx.accounts.taxonomy; 
    // let owner: &Signer = &ctx.accounts.owner; 
    
    if label.chars().count() > 24 {
        return Err(ErrorCode::LabelTooLong.into())
    }

    if label.chars().count() < 1 {
        return Err(ErrorCode::LabelTooShort.into())
    }

    // payload.owner = *owner.key; // * is dereferencing - get value of pointer
    payload.label = label;
    payload.parent = parent; //?

    Ok(()) //handle error
}

}

// Account context - Feed all accounts that will be interacted with
#[derive(Accounts)]
pub struct CreateTaxonomyCxt<'info> { //'info is rust lifetime - tells compiler that this account will be used for the duration of the program
    #[account(init, payer = payer, space = TaxonomyAccount::LEN)]
    pub taxonomy: Account<'info, TaxonomyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, 
    pub system_program: Program<'info, System>, //Program is part of prelude - ensure we're using actual system acct // TODO - is this unnecessary?
}

#[derive(Accounts)]
pub struct UpdateTaxonomyCxt<'info> { 
    #[account(mut, has_one = owner)] //anchor will reject if owner mismatch & notice mut on account
    pub taxonomy: Account<'info, TaxonomyAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, //mut?
}

// Account definition - more info at: https://lorisleiva.com/create-a-solana-dapp-from-scratch/structuring-our-tweet-account
#[account]
pub struct TaxonomyAccount {
    pub owner: Pubkey,
    pub label: String,
    pub parent: Option<Pubkey>,
}

const DISCRIMINATOR_LENGTH: usize = 8; //part of all of new accounts - stores type of account
const PUBLIC_KEY_LENGTH: usize = 32; //author
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const LABEL_LENGTH: usize = 24 * 4; // 50 chars max (subjective) * 4 bytes pser char

impl TaxonomyAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author
        + PUBLIC_KEY_LENGTH // Parent
        + STRING_LENGTH_PREFIX + LABEL_LENGTH;   
}

#[error_code]
pub enum ErrorCode {
    #[msg("Label can't be longer then 24 characters.")]
    LabelTooLong,
    #[msg("Label too short.")]
    LabelTooShort,
}