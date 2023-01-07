use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod deposit_game {
    use super::*;

    pub fn initialize(
                ctx: Context<Initialize>, 
                nonce: u8,
                ) -> ProgramResult {
        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.nonce = nonce;
        pool.game_count = 0;
        pool.game_finished = "0".to_string();
        
        Ok(())
    }
    
    pub fn create_game(ctx: Context<CreateGame>, nonce: u8, vault_nonce: u8, id: String, odd: u8, players: u8, bid: u64) -> ProgramResult {
        
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

    pub fn deposit(ctx: Context<Deposit>) -> ProgramResult {
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
    authority: UncheckedAccount<'info>,

    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    #[account(
        zero,
    )]
    pool: Box<Account<'info, Pool>>,

    owner: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id: String)]
pub struct CreateGame<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
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
    game: Box<Account<'info, Game>>,
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref(),
            "odd_vault".as_bytes(),
            id.as_bytes(),
        ],
        bump,
    )]
    vault: UncheckedAccount<'info>,
    signer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pool: Account<'info, Pool>,
    #[account(
        mut, 
        has_one = vault,
    )]
    game: Box<Account<'info, Game>>,
    #[account(
        mut,
        seeds = [
            pool.key().as_ref(),
            "odd_vault".as_bytes(),
            game.id.as_bytes(),
        ],
        bump = game.vault_nonce,
    )]
    vault: AccountInfo<'info>,
    #[account(mut)]
    depositor: AccountInfo<'info>,
    #[account(
        mut,
        constraint = game.fee_receiver == fee_receiver.key()
    )]
    fee_receiver: AccountInfo<'info>,
    #[account(
        seeds = [
            pool.key().as_ref(),
        ],
        bump = pool.nonce,
    )]
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
        space = 8 + 32 + 8 + 8 + 32
    )]
    deposit: Box<Account<'info, Deposit>>,
    signer: Signer<'info>,
    // token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Pool {
    pub game_count: u64,
    pub game_finished: String,
    pub nonce: u8,
    /// Priviledged account.
    pub authority: Pubkey,
}

#[account]
#[derive(Default)]
pub struct Game {
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
pub struct Deposit {
    depositor: Pubkey,
    deposit_date: u64,
    deposit_index: u64,
    game_id: String
}

#[error]
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