use anchor_lang::prelude::*;
use anchor_lang::solana_program;
// use anchor_lang::
declare_id!("4vnihPiwkRbNNa5WJCzPbp27exRGz3Lm1dzfaLXEEHcJ");

const SOLSENDER_SEED: &[u8] = b"sendsol";
const NEW_SEED: &[u8] = b"newseed";

#[program]
pub mod soltransfer {
    use super::*;

    pub fn send_sol(ctx: Context<SimpleSend>, amount: u64) -> Result<()> {
        let send_account = &mut ctx.accounts.sender;
        let recieve_account = &mut ctx.accounts.reciever;
        let amount_to_be_sent = amount;

        let txn = solana_program::system_instruction::transfer(
            &send_account.key(),
            &recieve_account.key(),
            amount_to_be_sent,
        );

        let acount_infos = [
            send_account.to_account_info(),
            recieve_account.to_account_info(),
        ];

        solana_program::program::invoke(&txn, &acount_infos)?;

        Ok(())
    }

    pub fn init_account(ctx: Context<InitTheAccount>) -> Result<()> {
        let contract_account = &mut ctx.accounts.program_account;
        let authority = &mut ctx.accounts.authority;
        contract_account.authority = authority.key();
        contract_account.value = 0;
        let (_pda, bump) = Pubkey::find_program_address(&[SOLSENDER_SEED], ctx.program_id);
        contract_account.bump = bump;

        Ok(())
    }
    pub fn send_sol_to_pda(ctx: Context<SendSolToProgram>, amount: u64) -> Result<()> {
        let send_acount = &mut ctx.accounts.sender;
        let program_account = &mut ctx.accounts.program_account;
        let amount_to_be_sent = amount;
        let (pda, _bump) = Pubkey::find_program_address(&[SOLSENDER_SEED], ctx.program_id);

        msg!(&send_acount.lamports().to_string());
        msg!(&program_account.value.to_string());
        msg!(&program_account.to_account_info().lamports().to_string());
        let txn = solana_program::system_instruction::transfer(
            &send_acount.key(),
            &pda,
            amount_to_be_sent,
        );

        let account_infos = [
            send_acount.to_account_info(),
            program_account.to_account_info(),
        ];

        solana_program::program::invoke(&txn, &account_infos)?;

        Ok(())
    }

    pub fn withdraw_from_pda(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let reciever_account = &mut ctx.accounts.reciever;
        let program_account = &mut ctx.accounts.program_account;
        let amount_to_be_sent = amount;

        **program_account.try_borrow_mut_lamports()? = program_account
            .lamports()
            .checked_sub(amount_to_be_sent)
            .ok_or(ProgramError::InvalidArgument)?;
        **reciever_account.try_borrow_mut_lamports()? = reciever_account
            .lamports()
            .checked_add(amount_to_be_sent)
            .ok_or(ProgramError::InvalidArgument)?;
        Ok(())
    }

    pub fn send_sys_pda_sol(ctx: Context<SendSysSolPda>, amount: u64) -> Result<()> {
        let sender_account = &mut ctx.accounts.sender_account;
        let reciever_account = &mut ctx.accounts.program_account;
        let amount_to_be_sent = amount;
        let (pda, _bump) = Pubkey::find_program_address(&[NEW_SEED], ctx.program_id);

        let txn = solana_program::system_instruction::transfer(
            &sender_account.key(),
            &pda,
            amount_to_be_sent,
        );

        let account_infos = [
            sender_account.to_account_info(),
            reciever_account.to_account_info(),
        ];

        msg!(
            "{} before sending sol balance",
            &sender_account.lamports().to_string()
        );
        solana_program::program::invoke(&txn, &account_infos)?;
        msg!(
            "{} after sending sol balance",
            &sender_account.lamports().to_string()
        );
        Ok(())
    }

    pub fn withdraw_sol_sys_pda(ctx: Context<WithdrawSysPdaSol>, amount: u64) -> Result<()> {
        let reciever_acount = &mut ctx.accounts.reciever_account;
        let (pda, bump) = Pubkey::find_program_address(&[NEW_SEED], ctx.program_id);
        let program_account = &mut ctx.accounts.program_account;

        let txn =
            solana_program::system_instruction::transfer(&pda, &reciever_acount.key(), amount);

        let account_infos = [
            program_account.to_account_info(),
            reciever_acount.to_account_info(),
        ];
        msg!(
            "{} before sending sol balance",
            &program_account.lamports().to_string()
        );
        solana_program::program::invoke_signed(&txn, &account_infos, &[&[NEW_SEED, &[bump]]])?;
        msg!(
            "{} after sending sol balance",
            &program_account.lamports().to_string()
        );
        Ok(())
    }
}

// Validators

// this one is for just sending sol from one account to the other
#[derive(Accounts)]
pub struct SimpleSend<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub reciever: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

// now this inits the required pda and sets the authority of the program
#[derive(Accounts)]
pub struct InitTheAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, seeds=[SOLSENDER_SEED], bump,  payer=authority, space=10000)]
    pub program_account: Account<'info, SendMoney>,
    system_program: Program<'info, System>,
}

// this allows for sol to be sent to the pda
#[derive(Accounts)]
pub struct SendSolToProgram<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut, seeds=[SOLSENDER_SEED], bump=program_account.bump)]
    pub program_account: Account<'info, SendMoney>,
    system_program: Program<'info, System>,
}

// this allows sol to be send from the pda
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub program_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub reciever: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

//This will allow to send money to a pda which is not owned by the program
#[derive(Accounts)]
pub struct SendSysSolPda<'info> {
    #[account(mut)]
    /// CHECK: It is ok this is just the info, dont want to init an account
    pub program_account: AccountInfo<'info>,
    #[account(mut)]
    pub sender_account: Signer<'info>,
    system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct WithdrawSysPdaSol<'info> {
    #[account(mut)]
    /// CHECK: It is ok this is just the info, dont want to init an account
    pub program_account: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: It is ok this is just the info, dont want to init an account
    pub reciever_account: AccountInfo<'info>,
    system_program: Program<'info, System>,
}

// Data Structs
#[account]
pub struct SendMoney {
    pub authority: Pubkey,
    pub value: u64,
    bump: u8,
}
// pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> Result<()> {
//         let ix = anchor_lang::solana_program::system_instruction::transfer(
//             &ctx.accounts.from.key(),
//             &ctx.accounts.to.key(),
//             amount,
//         );
//         anchor_lang::solana_program::program::invoke(
//             &ix,
//             &[
//                 ctx.accounts.from.to_account_info(),
//                 ctx.accounts.to.to_account_info(),
//             ],
//         )?; return Ok(());
