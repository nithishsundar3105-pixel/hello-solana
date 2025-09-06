use anchor_lang::prelude::*;

declare_id!("5Aen9GvdPNqqBPdfQiB59k6qZQKk2BxG6Ev8qNHXnnLD");
#[program]
pub mod hello_solana {
    use super::*;
    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()>{
        msg!("hello Solana");
        Ok(())
    }
    
    }
#[derive(Accounts)]   
pub struct SayHello {}