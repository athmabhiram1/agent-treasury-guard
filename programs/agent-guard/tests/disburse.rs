use anchor_lang::prelude::*;
use anchor_litesvm::{AnchorLiteSVM, AssertionHelpers, Signer, TestHelpers};

anchor_lang::declare_program!(agent_guard);

// NOTE: `anchor build` emits target/idl/agent_guard.json; declare_program reads it
// from <crate>/idls/agent_guard.json at compile time — `anchor test` copies it there
// automatically when tests live under programs/<name>/tests/.

fn ctx_with_program() -> anchor_litesvm::AnchorContext {
    AnchorLiteSVM::build_with_program(
        agent_guard::ID,
        include_bytes!("../../target/deploy/agent_guard.so"),
    )
}

fn fund_treasury_and_mandate(
    ctx: &mut anchor_litesvm::AnchorContext,
    authority: &anchor_litesvm::Keypair,
    agent: &anchor_litesvm::Keypair,
    payee: Pubkey,
) -> (Pubkey, Pubkey) {
    use anchor_litesvm::Signer;
    let (treasury, _) = Pubkey::find_program_address(
        &[b"treasury", authority.pubkey().as_ref()],
        &agent_guard::ID,
    );
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::CreateTreasury {
            authority: authority.pubkey(),
            treasury,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::CreateTreasury)
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[authority])
        .unwrap()
        .assert_success();
    let (mandate, _) = Pubkey::find_program_address(
        &[b"mandate", treasury.as_ref(), &1u64.to_le_bytes()],
        &agent_guard::ID,
    );
    let expiry = ctx.svm.get_current_slot() + 100_000;
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::SignMandate {
            authority: authority.pubkey(),
            treasury,
            agent: agent.pubkey(),
            mandate,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::SignMandate {
            nonce: 1,
            payees: vec![payee],
            expiry_slot: expiry,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[authority])
        .unwrap()
        .assert_success();
    ctx.airdrop(&treasury, 10_000_000_000).unwrap();
    (treasury, mandate)
}

fn disburse(
    ctx: &mut anchor_litesvm::AnchorContext,
    agent: &anchor_litesvm::Keypair,
    treasury: Pubkey,
    mandate: Pubkey,
    payee: Pubkey,
    amount: u64,
    idem: [u8; 32],
) -> anchor_litesvm::TransactionResult {
    let (record, _) =
        Pubkey::find_program_address(&[b"spent", mandate.as_ref(), &idem], &agent_guard::ID);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::AgentDisburse {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::AgentDisburse {
            amount,
            idem_key: idem,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[agent]).unwrap()
}

#[test]
fn disburse_happy_path_moves_exact_lamports() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    disburse(
        &mut ctx,
        &agent,
        treasury,
        mandate,
        payee,
        100_000_000,
        [7u8; 32],
    )
    .assert_success();
    ctx.svm.assert_sol_balance(&payee, 101_000_000);
    let (record, _) =
        Pubkey::find_program_address(&[b"spent", mandate.as_ref(), &[7u8; 32]], &agent_guard::ID);
    assert!(ctx.account_exists(&record));
}

#[test]
fn disburse_negatives_revert_with_codes() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let rogue = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);

    disburse(
        &mut ctx, &agent, treasury, mandate, rogue, 10_000, [1u8; 32],
    )
    .assert_failure()
    .assert_error("E05");
    disburse(
        &mut ctx,
        &agent,
        treasury,
        mandate,
        payee,
        600_000_000,
        [2u8; 32],
    )
    .assert_failure()
    .assert_error("E07");
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [3u8; 32],
    )
    .assert_success();
    // Replay surfaces as Anchor's init-collision on the spent PDA (framework error),
    // which is the on-chain E09 replay guard firing before the handler body.
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [3u8; 32],
    )
    .assert_failure();
}

#[test]
fn kill_freezes_and_unkill_restores() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);

    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::KillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(agent_guard::client::args::KillSwitch { kill: true })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [9u8; 32],
    )
    .assert_failure()
    .assert_error("E10");

    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::KillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(agent_guard::client::args::KillSwitch { kill: false })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [10u8; 32],
    )
    .assert_success();
}

#[test]
fn large_disburse_requires_single_use_approval() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);

    let over = 1_500_000_000u64;
    let idem = [42u8; 32];
    let (approval, _) =
        Pubkey::find_program_address(&[b"approval", mandate.as_ref(), &idem], &agent_guard::ID);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::HumanApproveLarge {
            authority: authority.pubkey(),
            treasury,
            mandate,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::HumanApproveLarge {
            amount: over,
            idem_key: idem,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();

    let (record, _) =
        Pubkey::find_program_address(&[b"spent", mandate.as_ref(), &idem], &agent_guard::ID);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::AgentDisburseLarge {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::AgentDisburseLarge {
            amount: over,
            idem_key: idem,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&agent])
        .unwrap()
        .assert_success();
    assert!(!ctx.account_exists(&approval));
}

fn approve_large(
    ctx: &mut anchor_litesvm::AnchorContext,
    authority: &anchor_litesvm::Keypair,
    treasury: Pubkey,
    mandate: Pubkey,
    amount: u64,
    idem: [u8; 32],
) -> Pubkey {
    let (approval, _) =
        Pubkey::find_program_address(&[b"approval", mandate.as_ref(), &idem], &agent_guard::ID);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::HumanApproveLarge {
            authority: authority.pubkey(),
            treasury,
            mandate,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::HumanApproveLarge {
            amount,
            idem_key: idem,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[authority])
        .unwrap()
        .assert_success();
    approval
}

fn disburse_large(
    ctx: &mut anchor_litesvm::AnchorContext,
    agent: &anchor_litesvm::Keypair,
    treasury: Pubkey,
    mandate: Pubkey,
    payee: Pubkey,
    approval: Pubkey,
    amount: u64,
    idem: [u8; 32],
) -> anchor_litesvm::TransactionResult {
    let (record, _) =
        Pubkey::find_program_address(&[b"spent", mandate.as_ref(), &idem], &agent_guard::ID);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::AgentDisburseLarge {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::AgentDisburseLarge {
            amount,
            idem_key: idem,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[agent]).unwrap()
}

#[test]
fn large_without_approval_fails() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let idem = [54u8; 32];
    let (approval, _) =
        Pubkey::find_program_address(&[b"approval", mandate.as_ref(), &idem], &agent_guard::ID);
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval, 1_500_000_000, idem,
    )
    .assert_failure();
}

#[test]
fn large_amount_mismatch_rejects_E12_approval_survives() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let idem = [52u8; 32];
    let approval = approve_large(&mut ctx, &authority, treasury, mandate, 1_500_000_000, idem);
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval, 1_400_000_000, idem,
    )
    .assert_failure()
    .assert_error("E12");
    assert!(ctx.account_exists(&approval));
}

#[test]
fn large_approval_single_use_reuse_fails() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let idem = [53u8; 32];
    let approval = approve_large(&mut ctx, &authority, treasury, mandate, 1_500_000_000, idem);
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval, 1_500_000_000, idem,
    )
    .assert_success();
    assert!(!ctx.account_exists(&approval));
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval, 1_500_000_000, idem,
    )
    .assert_failure();
}

#[test]
fn large_second_disburse_over_per_payee_cap_rejects_E06() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let first = [50u8; 32];
    let approval_first = approve_large(&mut ctx, &authority, treasury, mandate, 1_500_000_000, first);
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval_first, 1_500_000_000, first,
    )
    .assert_success();
    let second = [51u8; 32];
    let approval_second =
        approve_large(&mut ctx, &authority, treasury, mandate, 1_500_000_000, second);
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval_second, 1_500_000_000, second,
    )
    .assert_failure()
    .assert_error("E06");
}

#[test]
fn killed_large_disburse_rejects_E10() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let idem = [55u8; 32];
    let approval = approve_large(&mut ctx, &authority, treasury, mandate, 1_500_000_000, idem);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::KillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(agent_guard::client::args::KillSwitch { kill: true })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();
    disburse_large(
        &mut ctx, &agent, treasury, mandate, payee, approval, 1_500_000_000, idem,
    )
    .assert_failure()
    .assert_error("E10");
}

#[test]
fn kill_switch_agent_signed_rejects_E03() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, _) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::KillSwitch {
            kill_authority: agent.pubkey(),
            treasury,
        })
        .args(agent_guard::client::args::KillSwitch { kill: true })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&agent])
        .unwrap()
        .assert_failure()
        .assert_error("E03");
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::KillSwitch {
            kill_authority: agent.pubkey(),
            treasury,
        })
        .args(agent_guard::client::args::KillSwitch { kill: false })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&agent])
        .unwrap()
        .assert_failure()
        .assert_error("E03");
}

#[test]
fn fourth_disburse_in_window_rejects_E11_velocity() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    for key in [[60u8; 32], [61u8; 32], [62u8; 32]] {
        disburse(&mut ctx, &agent, treasury, mandate, payee, 10_000, key).assert_success();
    }
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [63u8; 32],
    )
    .assert_failure()
    .assert_error("E11");
}

#[test]
fn disburse_expired_mandate_rejects_E08() {
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let (treasury, mandate) = fund_treasury_and_mandate(&mut ctx, &authority, &agent, payee);
    let m: agent_guard::accounts::Mandate = ctx.get_account(&mandate).unwrap();
    ctx.svm.warp_to_slot(m.expiry_slot + 10);
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [31u8; 32],
    )
    .assert_failure()
    .assert_error("E08");
}
