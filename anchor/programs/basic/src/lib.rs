use anchor_lang::prelude::*;

declare_id!("6z68wfurCMYkZG51s1Et9BJEd9nJGUusjHXNt4dGbNNF");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_Poll(
        ctx: Context<InitializePoll>, 
        poll_id: u64,
        description: String,
        poll_start: u64,
        poll_end: u64,
    ) -> Result<()> {
        let Poll = &mut ctx.accounts.Poll;
        Poll.poll_id = poll_id;
        Poll.description = description;
        Poll.poll_start = poll_start;
        Poll.poll_end = poll_end;
        Poll.candidate_amount = 0;
        Ok(()) 
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + Poll::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub Poll: Account<'info, Poll>,


    /// CHECK
    pub system_program: AccountInfo<'info>,
}

#[account]
#[derive(InitSpace)] 
pub struct Poll {
    pub poll_id: u64,
    #[max_len(200)] 
    pub description: String,
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}


