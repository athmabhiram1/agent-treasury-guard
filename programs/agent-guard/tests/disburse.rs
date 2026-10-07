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

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsAgentDisburse {
    amount: u64,
    idem_key: [u8; 32],
}

impl anchor_lang::Discriminator for ArgsAgentDisburse {
    const DISCRIMINATOR: &'static [u8] = &[15, 108, 54, 166, 16, 152, 254, 9];
}

impl anchor_lang::InstructionData for ArgsAgentDisburse {}

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsHumanApproveLarge {
    amount: u64,
    idem_key: [u8; 32],
}

impl anchor_lang::Discriminator for ArgsHumanApproveLarge {
    const DISCRIMINATOR: &'static [u8] = &[180, 15, 40, 177, 23, 17, 172, 129];
}

impl anchor_lang::InstructionData for ArgsHumanApproveLarge {}

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsAgentDisburseLarge {
    amount: u64,
    idem_key: [u8; 32],
}

impl anchor_lang::Discriminator for ArgsAgentDisburseLarge {
    const DISCRIMINATOR: &'static [u8] = &[68, 254, 94, 191, 136, 124, 147, 20];
}

impl anchor_lang::InstructionData for ArgsAgentDisburseLarge {}

#[derive(anchor_lang::AnchorSerialize)]
struct ArgsKillSwitch {
    kill: bool,
}

impl anchor_lang::Discriminator for ArgsKillSwitch {
    const DISCRIMINATOR: &'static [u8] = &[189, 76, 222, 157, 130, 131, 241, 144];
}

impl anchor_lang::InstructionData for ArgsKillSwitch {}

struct IxCreateTreasury {
    authority: Pubkey,
    treasury: Pubkey,
    system_program: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxCreateTreasury {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.authority, true),
            anchor_lang::prelude::AccountMeta::new(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}

struct IxSignMandate {
    authority: Pubkey,
    treasury: Pubkey,
    agent: Pubkey,
    mandate: Pubkey,
    system_program: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxSignMandate {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.authority, true),
            anchor_lang::prelude::AccountMeta::new_readonly(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.agent, false),
            anchor_lang::prelude::AccountMeta::new(self.mandate, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}

struct IxAgentDisburse {
    agent: Pubkey,
    treasury: Pubkey,
    mandate: Pubkey,
    payee: Pubkey,
    spend_record: Pubkey,
    system_program: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxAgentDisburse {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.agent, true),
            anchor_lang::prelude::AccountMeta::new(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new(self.mandate, false),
            anchor_lang::prelude::AccountMeta::new(self.payee, false),
            anchor_lang::prelude::AccountMeta::new(self.spend_record, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}

struct IxAgentDisburseLarge {
    agent: Pubkey,
    treasury: Pubkey,
    mandate: Pubkey,
    payee: Pubkey,
    spend_record: Pubkey,
    approval: Pubkey,
    system_program: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxAgentDisburseLarge {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.agent, true),
            anchor_lang::prelude::AccountMeta::new(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new(self.mandate, false),
            anchor_lang::prelude::AccountMeta::new(self.payee, false),
            anchor_lang::prelude::AccountMeta::new(self.spend_record, false),
            anchor_lang::prelude::AccountMeta::new(self.approval, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}

struct IxHumanApproveLarge {
    authority: Pubkey,
    treasury: Pubkey,
    mandate: Pubkey,
    approval: Pubkey,
    system_program: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxHumanApproveLarge {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.authority, true),
            anchor_lang::prelude::AccountMeta::new_readonly(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.mandate, false),
            anchor_lang::prelude::AccountMeta::new(self.approval, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.system_program, false),
        ]
    }
}

struct IxKillSwitch {
    treasury: Pubkey,
    kill_authority: Pubkey,
}

impl anchor_lang::ToAccountMetas for IxKillSwitch {
    fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<anchor_lang::prelude::AccountMeta> {
        vec![
            anchor_lang::prelude::AccountMeta::new(self.treasury, false),
            anchor_lang::prelude::AccountMeta::new_readonly(self.kill_authority, true),
        ]
    }
}

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
        .accounts(IxCreateTreasury {
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
    let (mandate, _) = Pubkey::find_program_address(
        &[b"mandate", treasury.as_ref(), &1u64.to_le_bytes()],
        &agent_guard::ID,
    );
    let expiry = ctx.svm.get_current_slot() + 100_000;
    let ix = ctx
        .program()
        .accounts(IxSignMandate {
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
        .accounts(IxAgentDisburse {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsAgentDisburse {
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
        .accounts(IxKillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(ArgsKillSwitch { kill: true })
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
        .accounts(IxKillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(ArgsKillSwitch { kill: false })
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
        .accounts(IxHumanApproveLarge {
            authority: authority.pubkey(),
            treasury,
            mandate,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsHumanApproveLarge {
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
        .accounts(IxAgentDisburseLarge {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsAgentDisburseLarge {
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
        .accounts(IxHumanApproveLarge {
            authority: authority.pubkey(),
            treasury,
            mandate,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsHumanApproveLarge {
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
        .accounts(IxAgentDisburseLarge {
            agent: agent.pubkey(),
            treasury,
            mandate,
            payee,
            spend_record: record,
            approval,
            system_program: anchor_lang::system_program::ID,
        })
        .args(ArgsAgentDisburseLarge {
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
        .accounts(IxKillSwitch {
            kill_authority: authority.pubkey(),
            treasury,
        })
        .args(ArgsKillSwitch { kill: true })
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
        .accounts(IxKillSwitch {
            kill_authority: agent.pubkey(),
            treasury,
        })
        .args(ArgsKillSwitch { kill: true })
        .instruction()
        .unwrap();
    ctx.execute_instruction(ix, &[&agent])
        .unwrap()
        .assert_failure()
        .assert_error("E03");
    let ix = ctx
        .program()
        .accounts(IxKillSwitch {
            kill_authority: agent.pubkey(),
            treasury,
        })
        .args(ArgsKillSwitch { kill: false })
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
    let m: agent_guard::state::Mandate = ctx.get_account(&mandate).unwrap();
    ctx.svm.warp_to_slot(m.expiry_slot + 10);
    disburse(
        &mut ctx, &agent, treasury, mandate, payee, 10_000, [31u8; 32],
    )
    .assert_failure()
    .assert_error("E08");
}
