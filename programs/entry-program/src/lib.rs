use anchor_lang::prelude::*;

declare_id!("7BnsrUwQorEwpzLK5U9Kte9qPgWXzGPNyKBJJXdDEPHW"); 

#[program]
pub mod entry_program {
    use super::*;
   
    pub fn create_entry(ctx: Context<CreateEntryCxt>, arweave_id: String, taxonomy: Vec<Pubkey>, immutable: bool, archived: bool) -> Result<()> { //contexts + sides of fries 🐍 LABEL
        let payload: &mut Account<EntryAccount> = &mut ctx.accounts.entry; 
        let owner: &Signer = &ctx.accounts.owner; 

        if arweave_id.chars().count() != 43 {
            return Err(ErrorCode::IdIncorrectLength.into())
        }

        if taxonomy.len() > 3 {
            return Err(ErrorCode::TaxonomyTooLong.into())
        }

        payload.owner = *owner.key; // * is dereferencing - get value of pointer
        payload.taxonomy = taxonomy;
        payload.arweave_id = arweave_id;
        payload.immutable = immutable;
        payload.archived = archived;
        payload.version = 0;

        Ok(()) //handle error
}

    pub fn update_entry(ctx: Context<UpdateAssetCxt>, arweave_id: String, immutable: bool, taxonomy: Option<Vec<Pubkey>>, archived: Option<bool>) -> Result<()> { //contexts + sides of fries 🐍 LABEL
        // extract from context and write
        let payload: &mut Account<EntryAccount> = &mut ctx.accounts.entry; 

        if payload.immutable {
            return Err(ErrorCode::Immutable.into())
        }

        if arweave_id.chars().count() != 43 {
            return Err(ErrorCode::IdIncorrectLength.into())
        }

        if taxonomy.is_some() {
            let tax: &mut Vec<Pubkey> = &mut taxonomy.unwrap(); 
            if tax.len() > 3 {
                return Err(ErrorCode::TaxonomyTooLong.into())
            }
            else { payload.taxonomy = tax.to_vec() }
        }

        if payload.version > 254 {
            return Err(ErrorCode::InvalidVersion.into())
        }

        if archived.is_some() {
            payload.archived = archived.unwrap();
        }

        payload.arweave_id = arweave_id;
        payload.immutable = immutable;
        payload.version = payload.version + 1;

        Ok(()) //handle error
    }

}

#[derive(Accounts)]
pub struct CreateEntryCxt<'info> { //'info is rust lifetime - tells compiler that this account will be used for the duration of the program
    #[account(init, payer = payer, space = EntryAccount::LEN)]
    pub entry: Account<'info, EntryAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, 
    pub system_program: Program<'info, System>, //Program is part of prelude - ensure we're using actual system acct // TODO - is this unnecessary?
}

#[derive(Accounts)]
pub struct UpdateAssetCxt<'info> { 
    #[account(mut, has_one = owner)] //anchor will reject if owner mismatch & notice mut on account
    pub entry: Account<'info, EntryAccount>,
    #[account(mut)]
    pub payer: Signer<'info>, //mutable since their balance will change
    pub owner: Signer<'info>, //mut?
}


// Account definition - more info at: https://lorisleiva.com/create-a-solana-dapp-from-scratch/structuring-our-tweet-account
#[account]
pub struct EntryAccount {
    pub owner: Pubkey,
    pub arweave_id: String,
    pub taxonomy: Vec<Pubkey>,   
    pub immutable: bool,
    pub archived: bool,
    pub version: u8,
}

const DISCRIMINATOR_LENGTH: usize = 8; //part of all of new accounts - stores type of account
const PUBLIC_KEY_LENGTH: usize = 32; //author
const PREFIX_LENGTH: usize = 4; // Stores the size of the string.
const ID_LENGTH: usize = 64 * 4; // SHA256 * 4 bytes pser char
const TAXONOMY_LENGTH: usize = 32 * 3;
const ARCHIVED_LENGTH: usize = 1; // Boolean
const IMMUTABLE_LENGTH: usize = 1; // Boolean
const VERSION_LENGTH: usize = 1; // u8

impl EntryAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner pubkey
        + PREFIX_LENGTH + ID_LENGTH // Arweave ID - string
        + PREFIX_LENGTH + TAXONOMY_LENGTH // Taxonomy - vec of pubkeys max 3
        + ARCHIVED_LENGTH
        + IMMUTABLE_LENGTH
        + VERSION_LENGTH;   
    }

#[error_code]
pub enum ErrorCode {
    #[msg("arweave_id is not the correct length.")]
    IdIncorrectLength,
    #[msg("taxonomy cannot exceed 3 items.")]
    TaxonomyTooLong,
    #[msg("this program is immutable.")]
    Immutable,
    #[msg("this program no longer has version remaining.")]
    InvalidVersion,

}