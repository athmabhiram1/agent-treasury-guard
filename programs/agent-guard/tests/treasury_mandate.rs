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

fn treasury_pda(program_id: &Pubkey, authority: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"treasury", authority.as_ref()], program_id).0
}

fn mandate_pda(program_id: &Pubkey, treasury: &Pubkey, nonce: u64) -> Pubkey {
    Pubkey::find_program_address(
        &[b"mandate", treasury.as_ref(), &nonce.to_le_bytes()],
        program_id,
    )
    .0
}

fn create_treasury(
    ctx: &mut anchor_litesvm::AnchorContext,
    authority: &anchor_litesvm::Keypair,
) -> Pubkey {
    use anchor_litesvm::Signer;
    let treasury = treasury_pda(&agent_guard::ID, &authority.pubkey());
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
    treasury
}

#[test]
fn create_treasury_sets_fields() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let treasury = create_treasury(&mut ctx, &authority);
    let t: agent_guard::accounts::Treasury = ctx.get_account(&treasury).unwrap();
    assert_eq!(t.authority, authority.pubkey());
    assert_eq!(t.kill_authority, authority.pubkey());
    assert_eq!(t.budget, 5_000_000_000);
    assert!(!t.killed);
}

#[test]
fn create_treasury_reinit_fails() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let treasury = create_treasury(&mut ctx, &authority);
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
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_failure();
}

#[test]
fn sign_mandate_stores_values_and_rejects_agent_key() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let treasury = create_treasury(&mut ctx, &authority);
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let expiry = ctx.svm.get_current_slot() + 10_000;
    let mandate = mandate_pda(&agent_guard::ID, &treasury, 1);

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
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();
    let m: agent_guard::accounts::Mandate = ctx.get_account(&mandate).unwrap();
    assert_eq!(m.agent, agent.pubkey());
    assert_eq!(m.payees, vec![payee]);
    assert_eq!(m.per_payee_cap, 2_000_000_000);
    assert_eq!(m.tx_ceiling, 500_000_000);
    assert_eq!(m.large_threshold, 1_000_000_000);

    let rogue_mandate = mandate_pda(&agent_guard::ID, &treasury, 2);
    let ix = ctx
        .program()
        .accounts(agent_guard::client::accounts::SignMandate {
            authority: agent.pubkey(),
            treasury,
            agent: agent.pubkey(),
            mandate: rogue_mandate,
            system_program: anchor_lang::system_program::ID,
        })
        .args(agent_guard::client::args::SignMandate {
            nonce: 2,
            payees: vec![payee],
            expiry_slot: expiry,
        })
        .instruction()
        .unwrap();
    let res = ctx.execute_instruction(ix, &[&agent]).unwrap();
    res.assert_failure();
    res.assert_error("E03");
}

#[test]
fn sign_mandate_same_nonce_reuse_fails() {
    use anchor_litesvm::Signer;
    let mut ctx = ctx_with_program();
    let authority = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let agent = ctx.svm.create_funded_account(10_000_000_000).unwrap();
    let treasury = create_treasury(&mut ctx, &authority);
    let payee = ctx.svm.create_funded_account(1_000_000).unwrap().pubkey();
    let expiry = ctx.svm.get_current_slot() + 10_000;
    let mandate = mandate_pda(&agent_guard::ID, &treasury, 1);
    let sign = |ctx: &mut anchor_litesvm::AnchorContext| {
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
        ctx.execute_instruction(ix, &[&authority]).unwrap()
    };
    sign(&mut ctx).assert_success();
    sign(&mut ctx).assert_failure();
}
