use anchor_lang::prelude::*;

anchor_gen::generate_cpi_interface!(idl_path = "idl.json",);

declare_id!("GDDMwNyyx8uB6zrqwBFHjLLG3TBYk2F8Az4yrQC5RzMp");

/// Encodes a string into an array of bytes fixed with 32 length.
#[inline(always)]
fn encode_string(alias: &str) -> Vec<u8> {
    let alias_bytes = alias.as_bytes();
    let mut encoded = Vec::with_capacity(alias_bytes.len());
    assert!(alias_bytes.len() <= 32);
    for byte in alias_bytes.iter() {
        encoded.push(*byte);
    }
    encoded
}

pub fn derive_sequence_account_address(seed: &str, authority: &Pubkey) -> (Pubkey, u8) {
    let encoded_seed = encode_string(seed);

    let (address, bump) =
        Pubkey::find_program_address(&[&encoded_seed, authority.as_ref()], &crate::id());

    (address, bump)
}

pub mod instructions {
    use crate::accounts::{CheckAndSetSequenceNumber, Initialize, ResetSequenceNumber};
    use anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, system_program},
        InstructionData,
    };

    pub fn initialize(
        sequence_account: &Pubkey,
        authority: &Pubkey,
        seed: String,
        bump: u8,
    ) -> Instruction {
        let accounts = Initialize {
            sequence_account: *sequence_account,
            authority: *authority,
            system_program: system_program::id(),
        };

        Instruction {
            program_id: crate::id(),
            data: crate::instruction::Initialize {
                _bump: bump,
                _sym: seed,
            }
            .data(),
            accounts: accounts.to_account_metas(None),
        }
    }

    pub fn check_and_set_sequence_number(
        sequence_account: &Pubkey,
        authority: &Pubkey,
        sequence_num: u64,
    ) -> Instruction {
        let accounts = CheckAndSetSequenceNumber {
            sequence_account: *sequence_account,
            authority: *authority,
        };

        Instruction {
            program_id: crate::id(),
            data: crate::instruction::CheckAndSetSequenceNumber {
                _sequence_num: sequence_num,
            }
            .data(),
            accounts: accounts.to_account_metas(None),
        }
    }

    pub fn reset_sequence_number(
        sequence_account: &Pubkey,
        authority: &Pubkey,
        sequence_num: u64,
    ) -> Instruction {
        let accounts = ResetSequenceNumber {
            sequence_account: *sequence_account,
            authority: *authority,
        };

        Instruction {
            program_id: crate::id(),
            data: crate::instruction::ResetSequenceNumber {
                _sequence_num: sequence_num,
            }
            .data(),
            accounts: accounts.to_account_metas(None),
        }
    }
}
