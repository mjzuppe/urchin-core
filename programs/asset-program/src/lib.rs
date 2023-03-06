use anchor_lang::prelude::*;

declare_id!("H1YXak97J5CiuEJUjdMQg9YSEkCRLHpKQA3kuUEUKgYz"); 

// Program entrypoint - this is the main function that is called when the program is invoked
#[program]
pub mod asset_program { //🐍 case
    use super::*;
   
    pub fn create_asset(ctx: Context<CreateAssetCxt>, arweave_id: String, immutable: bool, archived: bool) -> Result<()> { //contexts + sides of fries 🐍 LABEL
        let payload: &mut Account<AssetAccount> = &mut ctx.accounts.asset; 
        let owner: &Signer = &ctx.accounts.owner; 

        if arweave_id.chars().count() != 64 {
            return Err(ErrorCode::IdIncorrectLength.into())
        }

        payload.owner = *owner.key; // * is dereferencing - get value of pointer
        payload.arweave_id = arweave_id;
        payload.immutable = immutable;
        payload.archived = archived;

    Ok(()) //handle error
}
    pub fn update_asset(ctx: Context<UpdateAssetCxt>, arweave_id: String, immutable: bool, archived: Option<bool>) -> Result<()> { //contexts + sides of fries 🐍 LABEL
        // extract from context and write
        let payload: &mut Account<AssetAccount> = &mut ctx.accounts.asset; 

        if payload.immutable {
            return Err(ErrorCode::Immutable.into())
        }

        if arweave_id.chars().count() != 64 {
            return Err(ErrorCode::IdIncorrectLength.into())
        }

        if archived.is_some() {
            payload.archived = archived.unwrap();
        }

        payload.arweave_id = arweave_id;
        payload.immutable = immutable;
   

        Ok(()) //handle error
    }




}

// Account context - Feed all accounts that will be interacted with
#[derive(Accounts)]
pub struct CreateAssetCxt<'info> { //'info is rust lifetime - tells compiler that this account will be used for the duration of the program
    #[account(init, payer = payer, space = AssetAccount::LEN)]
    pub asset: Account<'info, AssetAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, 
    pub system_program: Program<'info, System>, //Program is part of prelude - ensure we're using actual system acct // TODO - is this unnecessary?
}

#[derive(Accounts)]
pub struct UpdateAssetCxt<'info> { 
    #[account(mut, has_one = owner)] //anchor will reject if owner mismatch & notice mut on account
    pub asset: Account<'info, AssetAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, //mut?
}

// Account definition - more info at: https://lorisleiva.com/create-a-solana-dapp-from-scratch/structuring-our-tweet-account
#[account]
pub struct AssetAccount {
    pub owner: Pubkey,
    pub arweave_id: String,
    pub immutable: bool,
    pub archived: bool,
}

const DISCRIMINATOR_LENGTH: usize = 8; //part of all of new accounts - stores type of account
const PUBLIC_KEY_LENGTH: usize = 32; //author
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const ID_LENGTH: usize = 64 * 4; // SHA256 * 4 bytes pser char
const ARCHIVED_LENGTH: usize = 1; // Boolean
const IMMUTABLE_LENGTH: usize = 1; // u8

impl AssetAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner pubkey
        + STRING_LENGTH_PREFIX + ID_LENGTH // Arweave ID - string
        + ARCHIVED_LENGTH
        + IMMUTABLE_LENGTH;   }

#[error_code]
pub enum ErrorCode {
    #[msg("arweave_id is not the correct length.")]
    IdIncorrectLength,
    #[msg("this program is immutable.")]
    Immutable,

}