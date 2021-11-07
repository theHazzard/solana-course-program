use anchor_lang::prelude::*;

declare_id!("Ac9EmrmiQWgs7hSLLALjcpeQsSpw9bHXPXAC7GawxKRM");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }
  
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    let item = ItemStruct {
        gif_link: gif_link.to_string(),
        user_address: *base_account.to_account_info().key,
        votes: 0,
    };
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }

  pub fn update_item(ctx: Context<UpdateItem>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;

    let mut item = base_account.gif_list.iter_mut().find(|elem| elem.gif_link ==  gif_link).unwrap();

    item.votes += 1;
    Ok(())
  }

  pub fn send_sol(ctx: Context<SendSol>, amount: u64) -> ProgramResult {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.from.key(),
        &ctx.accounts.to.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.to.to_account_info(),
        ],
    )
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct UpdateItem<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct SendSol<'info> {
    #[account(mut)]
    from: Signer<'info>,
    #[account(mut)]
    to: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub votes: u32,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}