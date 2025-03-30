use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(Debug, Default)]
pub struct Counter {
    pub count: u64,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let accounts_iter = &mut accounts.iter();
    let counter_account = next_account_info(accounts_iter)?;
    if counter_account.owner != program_id {
        msg!("Counter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    if instruction_data.is_empty() {
        msg!("Reading counter value");
        let counter_data = counter_account.try_borrow_data()?;
        let count = if counter_data.len() >= 8 {
            let bytes = &counter_data[0..8];
            u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            0
        };
        msg!("Current count: {}", count);
    } else {
        msg!("Incrementing counter value");
        let mut counter_data = counter_account.try_borrow_mut_data()?;

        let mut count = if counter_data.len() >= 8 {
            let bytes = &counter_data[0..8];
            u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])
        } else {
            0
        };

        count += 1;
        msg!("New count: {}", count);

        if counter_data.len() >= 8 {
            counter_data[0..8].copy_from_slice(&count.to_le_bytes());
        }
    }

    Ok(())
}
