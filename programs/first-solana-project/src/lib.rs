use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
declare_id!("BKHLVt2rmFftGD8wGEC8nLnXkNutLHpfiLPu4WKxR7wg");


#[program]
pub mod first_solana_project {
    use super::*;
    
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            upvotes: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn upvote_gif(ctx: Context<UpvoteGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let link = gif_link.to_string();

        for gif in base_account.gif_list.iter_mut() {
            match &gif.gif_link {
                link => gif.upvotes += 1,
            }
        }
        Ok(())
    }

    pub fn downvote_gif(ctx: Context<DownvoteGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let link = gif_link.to_string();

        for gif in base_account.gif_list.iter_mut() {
            match &gif.gif_link {
                link => gif.upvotes -= 1,
            }
        }
        Ok(())
    }

    pub fn delete_gif(ctx: Context<DeleteGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let link = gif_link.to_string();

        let index = base_account.gif_list.iter().position(|x| *x.gif_link == link).unwrap();
        base_account.gif_list.remove(index);

        base_account.total_gifs -= 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct DownvoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct DeleteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: u64,
}

#[account]
#[derive(Default)]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}