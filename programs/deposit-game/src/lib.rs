use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};

declare_id!("5YZgcGBxfWRDmFX9oiXVQwTZH4QMVG7h6T11kkujbXC4");

#[program]
pub mod deposit_game {
    use super::*;

    pub fn initialize(
                ctx: Context<Initialize>, 
                nonce: u8,
                ) -> Result<()> {
        // let pool = &mut ctx.accounts.pool;
        // pool.authority = ctx.accounts.authority.key();
        // pool.nonce = nonce;
        // pool.game_count = 0;
        // pool.game_finished = "0".to_string();
        
        Ok(())
    }
    
    pub fn create_game(ctx: Context<CreateGame>, nonce: u8, vault_nonce: u8, id: String, odd: u8, players: u8, bid: u64) -> Result<()> {
        
        let game = &mut ctx.accounts.game;
        game.authority = ctx.accounts.pool.authority;
        game.finished = false;
        game.deposited = 0;
        game.odd = odd;
        game.players = players;
        game.bid = bid;
        game.vault = ctx.accounts.vault.key();
        game.creator = ctx.accounts.signer.key();
        game.nonce = nonce;
        game.vault_nonce = vault_nonce;
        game.fee_receiver = ctx.accounts.signer.key();
        game.id = id;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        let game = &mut ctx.accounts.game;
        let deposit = &mut ctx.accounts.deposit;
        if game.finished == true {
            return Err(ErrorCode::FinishedGame.into());
        }
        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.fee_receiver.key(), 
                                    game.bid.checked_mul(3 as u64).unwrap().checked_div(100 as u64).unwrap());
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.fee_receiver.to_account_info(), 
                                                            ])?;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
                                    &ctx.accounts.depositor.key(), 
                                    &ctx.accounts.vault.key(), 
                                    game.bid.checked_mul(97 as u64).unwrap().checked_div(100 as u64).unwrap());
        anchor_lang::solana_program::program::invoke(&ix, &[
                                                                ctx.accounts.depositor.to_account_info(), 
                                                                ctx.accounts.vault.to_account_info(), 
                                                            ])?;

        let current_time = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        deposit.depositor = ctx.accounts.depositor.key();
        deposit.deposit_date = current_time;
        deposit.deposit_index = game.deposited + 1;
        deposit.game_id = game.id.clone();

        game.deposited = game.deposited + 1;

        if game.deposited == game.players {
            game.finished = true;

            ctx.accounts.pool.game_finished = game.id.clone();
        }
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct Initialize<'info> {
    /// CHECK: pool authority. checked
    authority: UncheckedAccount<'info>,

    // #[account(
    //     seeds = [
    //         pool.to_account_info().key.as_ref()
    //     ],
    //     bump = nonce,
    // )]
    // /// CHECK: pool signer. checked
    // pool_signer: UncheckedAccount<'info>,

    // #[account(
    //     zero,
    // )]
    // pool: Box<Account<'info, PoolAccount>>,

    #[account(mut)]
    owner: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pool: Account<'info, PoolAccount>,
    #[account(
        init,
        payer = signer,
        seeds = [
            pool.key().as_ref(),
            "odd_game".as_bytes(),
            id.as_bytes(),
        ],
        bump,
        space = 8 + 32 + 1 + 1 + 1 + 8 + 32 + 32 + 1 + 1 + 32 + 1 + 32 + 32
    )]
    game: Box<Account<'info, GameAccount>>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "odd_vault".as_bytes(),
            id.as_bytes(),
        ],
        bump,
    )]
    /// CHECK: deposit sol vault. checked
    vault: UncheckedAccount<'info>,
    #[account(mut)]
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pool: Account<'info, PoolAccount>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, GameAccount>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "odd_vault".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    /// CHECK: deposit vault. checked
    vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: depositor. checked
    depositor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = game.fee_receiver == fee_receiver.key()
    )]
    /// CHECK: fee receiver. checked
    fee_receiver: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
    /// CHECK: pool signer. checked
    pool_signer: UncheckedAccount<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [
            depositor.key().as_ref(),
            "game".as_bytes(),
            game.id.as_bytes(),
        ],
        bump,
        space = 8 + 32 + 8 + 1 + 32
    )]
    deposit: Box<Account<'info, DepositAccount>>,
    #[account(mut)]
    signer: Signer<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct PoolAccount {
    pub game_count: u64,
    pub game_finished: String,
    pub nonce: u8,
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct GameAccount {
    pub vault: Pubkey,
    pub finished: bool,
    pub odd: u8,
    pub players: u8,
    pub bid: u64,
    pub creator: Pubkey,
    pub fee_receiver: Pubkey,
    pub deposited: u8,
    pub nonce: u8,
    pub id: String,
    pub vault_nonce: u8,
    pub last_hash: [u8; 32],
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct DepositAccount {
    depositor: Pubkey,
    deposit_date: u64,
    deposit_index: u8,
    game_id: String
}

#[error_code]
pub enum ErrorCode {
    #[msg("Minimize deposit amount is 0.000001 SOL.")]
    MinDepositAmount,
    #[msg("This address deposited already.")]
    AlreadyDeposit,
    #[msg("Depositor address does not registered.")]
    DepositorNotMatch,
    #[msg("Finished Game.")]
    FinishedGame,
}