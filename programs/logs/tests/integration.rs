mod utils;

use anchor_lang::Discriminator;
use solana_program::message::Message;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transaction::Transaction,
};

use crate::utils::setup_svm;

#[test]
fn test_initialize() {
    let mut svm = setup_svm();

    let signer = Keypair::new();
    svm.airdrop(&signer.pubkey(), 100_000).unwrap();

    let instruction = Instruction {
        program_id: logs::ID,
        accounts: vec![],
        data: logs::instruction::Initialize::DISCRIMINATOR.to_vec(),
    };
    let tx = Transaction::new(
        &[&signer],
        Message::new(&[instruction], Some(&signer.pubkey())),
        svm.latest_blockhash(),
    );
    let tx_res = svm.send_transaction(tx).unwrap();

    assert!(tx_res
        .logs
        .iter()
        .any(|log| log.contains(&format!("Greetings from: {}", logs::ID))));
}
