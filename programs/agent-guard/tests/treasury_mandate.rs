use anchor_lang::prelude::*;
use anchor_litesvm::{AnchorLiteSVM, AssertionHelpers, Signer, TestHelpers};

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsCreateTreasury;

impl anchor_lang::Discriminator for ArgsCreateTreasury {
    const DISCRIMINATOR: &'static [u8] = &[254, 98, 217, 51, 25, 88, 140, 45];
}

impl anchor_lang::InstructionData for ArgsCreateTreasury {}

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsSignMandate {
    nonce: u64,
    payees: Vec<Pubkey>,
    expiry_slot: u64,
}

impl anchor_lang::Discriminator for ArgsSignMandate {
    const DISCRIMINATOR: &'static [u8] = &[180, 207, 74, 168, 214, 174, 238, 189];
}

impl anchor_lang::InstructionData for ArgsSignMandate {}

fn program_bytes() -> Vec<u8> {
    if let Ok(path) = std::env::var("AGENT_GUARD_SO") {
        return std::fs::read(&path)
            .unwrap_or_else(|_| panic!("missing Solana program binary at {}", path));
    }
    for candidate in [
        "../../target/deploy/agent_guard.so",
        "target/deploy/agent_guard.so",
    ] {
        if let Ok(bytes) = std::fs::read(candidate) {
            return bytes;
        }
    }
    panic!("missing Solana program binary; run `anchor build` first")
}

fn ctx_with_program() -> anchor_litesvm::AnchorContext {
    AnchorLiteSVM::build_with_program(agent_guard::ID, &program_bytes())
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
        .accounts(agent_guard::CreateTreasury {
            authority: authority.pubkey(),
            treasury,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsCreateTreasury)
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
    let t: agent_guard::state::Treasury = ctx.get_account(&treasury).unwrap();
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
        .accounts(agent_guard::CreateTreasury {
            authority: authority.pubkey(),
            treasury,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsCreateTreasury)
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
        .accounts(agent_guard::SignMandate {
            authority: authority.pubkey(),
            treasury,
            agent: agent.pubkey(),
            mandate,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsSignMandate {
            nonce: 1,
            payees: vec![payee],
            expiry_slot: expiry,
        })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&authority])
        .unwrap()
        .assert_success();
    let m: agent_guard::state::Mandate = ctx.get_account(&mandate).unwrap();
    assert_eq!(m.agent, agent.pubkey());
    assert_eq!(m.payees, vec![payee]);
    assert_eq!(m.per_payee_cap, 2_000_000_000);
    assert_eq!(m.tx_ceiling, 500_000_000);
    assert_eq!(m.large_threshold, 1_000_000_000);

    let rogue_mandate = mandate_pda(&agent_guard::ID, &treasury, 2);
    let ix = ctx
        .program()
        .accounts(agent_guard::SignMandate {
            authority: agent.pubkey(),
            treasury,
            agent: agent.pubkey(),
            mandate: rogue_mandate,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsSignMandate {
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
            .accounts(agent_guard::SignMandate {
                authority: authority.pubkey(),
                treasury,
                agent: agent.pubkey(),
                mandate,
                system_program: anchor_lang::system_program::ID,
            })
            .args(ArgsSignMandate {
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
