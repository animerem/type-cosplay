use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("12nnnKByohts2zHU5tbfi18WjxYy5YrrvoZdRdEkBxgw");

#[program]
pub mod type_cosplay {
    use super::*;

    pub fn initialize_admin(ctx: Context<Initialize>) -> Result<()> {
        let admin_config = AdminConfig {
            admin: ctx.accounts.payer.key(),
        };

        let serialized_data = admin_config.try_to_vec()?;
        let space = serialized_data.len();
        let lamports = Rent::get()?.minimum_balance(space);

        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &ctx.accounts.new_account.key(),
            lamports,
            space as u64,
            &ctx.program_id,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.new_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // Запись сериализованных данных в аккаунт
        let mut data = ctx.accounts.new_account.data.borrow_mut();
        data[..space].copy_from_slice(&serialized_data);

        Ok(())
    }

    pub fn initialize_user(ctx: Context<Initialize>) -> Result<()> {
        let user_data = User {
            user: ctx.accounts.payer.key(),
        };
    
        let serialized_data = user_data.try_to_vec()?;
        let space = serialized_data.len();
        let lamports = Rent::get()?.minimum_balance(space);
    
        let ix = anchor_lang::solana_program::system_instruction::create_account(
            &ctx.accounts.payer.key(),
            &ctx.accounts.new_account.key(),
            lamports,
            space as u64,
            &ctx.program_id,
        );
    
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.new_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;
    
        ctx.accounts
            .new_account
            .data
            .borrow_mut()[..space]
            .copy_from_slice(&serialized_data);
    
        msg!("User: {}", ctx.accounts.payer.key());
    
        Ok(())
    }
    

    pub fn update_admin(ctx: Context<UpdateAdmin>) -> Result<()> {
        let mut account = AdminConfig::try_from_slice(&ctx.accounts.admin_config.data.borrow())
            .map_err(|_| ProgramError::InvalidAccountData)?;
    
        if ctx.accounts.admin.key() != account.admin {
            return Err(ProgramError::IllegalOwner.into());
        }
    
        account.admin = ctx.accounts.admin.key();
        account.serialize(&mut *ctx.accounts.admin_config.data.borrow_mut())?;
    
        msg!("Admin updated to {}", account.admin);
    
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub new_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub admin_config: UncheckedAccount<'info>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AdminConfig {
    pub admin: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct User {
    pub user: Pubkey,
}
