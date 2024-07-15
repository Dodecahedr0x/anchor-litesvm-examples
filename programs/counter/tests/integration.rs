mod utils;

use anchor_lang::{AnchorDeserialize, Discriminator};
use counter::Counter;
use solana_program::message::Message;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};

use crate::utils::setup_svm;

#[test]
fn test_increment_many_times() {
    let mut svm = setup_svm();

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 100_000_000).unwrap();

    let (counter_key, _) =
        Pubkey::find_program_address(&[b"counter", signer.pubkey().as_ref()], &counter::ID);

    for i in 1..100_u64 {
        let instruction = Instruction {
            program_id: counter::ID,
            accounts: vec![
                AccountMeta {
                    pubkey: counter_key,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: signer.pubkey(),
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: solana_program::system_program::ID,
                    is_signer: false,
                    is_writable: false,
                },
            ],
            data: [
                counter::instruction::IncrementCounter::DISCRIMINATOR.to_vec(),
                i.to_be_bytes().to_vec(),
            ]
            .concat(),
        };
        let tx = Transaction::new(
            &[&signer],
            Message::new(&[instruction], Some(&signer.pubkey())),
            svm.latest_blockhash(),
        );

        svm.send_transaction(tx).unwrap();

        let raw_data = svm.get_account(&counter_key).unwrap().data;
        let data = &raw_data.as_slice()[8..]; // Exclude discriminator
        let counter_account = Counter::try_from_slice(data).unwrap();
        assert_eq!(counter_account.value, i);
    }
}
