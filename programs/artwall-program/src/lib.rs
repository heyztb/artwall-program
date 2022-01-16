use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod artwall_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let artwall_account = &mut ctx.accounts.artwall_account;
        artwall_account.count = 0;
        Ok(())
    }

    pub fn submit_art(ctx: Context<SubmitArt>, data: String) -> ProgramResult {
        let artwall_account = &mut.ctx.accounts.artwall_account;
        let user = &mut ctx.accounts.user;

        let art = ArtStruct {
            data: data.to_string(),
            appreciation_count: 0,
            submitted_by: *user.to_account_info().key,
        };

        artwall_account.art_collection.push(art);
        artwall_account.count += 1;

        Ok(())
    }

    pub fn appreciate_art(ctx: Context<AppreciateArt>, index: String) -> ProgramResult {
        let artwall_account = &mut ctx.accounts.artwall_account;
        if let Ok(idx) = index.parse::<usize>() {
            let art = &mut artwall_account.art_collection[idx];
            art.appreciation_count += 1;
        } else {
            return Err(ProgramError::InvalidArgument);
        }
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize {
    #[account(init, payer = user, space = 9000)]
    pub artwall_account: Account<'info, ArtWallAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub struct ArtWallAccount {
    pub count: u64,
    pub art_collection: Vec<ArtStruct>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ArtStruct {
    pub data: String,
    pub appreciation_count: u64,
    pub submitted_by: Pubkey
}

pub struct SubmitArt<'info> {
    #[account(mut)]
    pub artwall_account: Account<'info, ArtWallAccount>,
    #[account(mut)]
    pub user: Singer<'info>
}

pub struct AppreciateArt<'info> {
    #[account(mut)]
    pub artwall_account: Account<'info, ArtWallAccount>
}