#![cfg_attr(target_arch = "bpf", no_std)]

use pinocchio::{AccountView, Address, ProgramResult, nostd_panic_handler, no_allocator, program_entrypoint};
use solana_program_log::log;

nostd_panic_handler!();
no_allocator!();

program_entrypoint!(process_instruction);

#[unsafe(no_mangle)]
fn process_instruction(
    _program_id: &Address,     // Address of the account the program was loaded into
    _accounts: &[AccountView], // All accounts required to process the instruction
    _instruction_data: &[u8],  // Serialized instruction-specific data
) -> ProgramResult {
    log("Hello, World!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use mollusk_svm::{Mollusk, result::Check};
    use solana_instruction::Instruction;

    #[test]
    pub fn hello_world() {
        let mollusk = Mollusk::new(&[2u8;32].into(), "target/bpfel-unknown-none/release/lib{{crate_name}}");
        mollusk.process_and_validate_instruction(&Instruction {
            program_id: [2u8;32].into(),
            accounts: vec![],
            data: vec![]
        }, &vec![], &[
            Check::success()
        ]);
    }
}