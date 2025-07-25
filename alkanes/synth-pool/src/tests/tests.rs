use super::*;
use alkanes_support::{
    context::Context,
    id::AlkaneId,
    parcel::{AlkaneTransfer, AlkaneTransferParcel},
};
use wasm_bindgen_test::*;
use anyhow::Result;

fn alkane_id(s: &str) -> AlkaneId {
    let mut block_bytes = [0u8; 16];
    let s_bytes = s.as_bytes();
    let len = s_bytes.len().min(16);
    block_bytes[..len].copy_from_slice(&s_bytes[..len]);
    let block = u128::from_le_bytes(block_bytes);
    AlkaneId { block, tx: 0 }
}

#[wasm_bindgen_test]
fn test_add_liquidity() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(10);
    logic.set_admin_fee(1);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let lp_balance = logic.balance_of(&liquidity_provider);
    
    std::println!("   └─ LP Balance: {}", lp_balance);
    assert!(lp_balance > 0);
    
    std::println!("✅ Add liquidity test passed");
    Ok(())
}

#[wasm_bindgen_test]
fn test_remove_liquidity_imbalance() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(10);
    logic.set_admin_fee(1);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let lp_balance = logic.balance_of(&liquidity_provider);
    assert!(lp_balance > 0);

    let context = Context {
        caller: liquidity_provider,
        ..Default::default()
    };

    logic.context = context;
    logic.remove_liquidity_imbalance(vec![100_000, 200_000], lp_balance)?;

    let lp_balance_after = logic.balance_of(&liquidity_provider);
    assert!(lp_balance_after < lp_balance);

    std::println!("✅ Remove liquidity imbalance test passed");
    Ok(())
}

#[wasm_bindgen_test]
fn test_remove_liquidity_one_coin() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(10);
    logic.set_admin_fee(1);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let lp_balance = logic.balance_of(&liquidity_provider);
    assert!(lp_balance > 0);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: logic.context.myself,
                value: lp_balance / 2,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.remove_liquidity_one_coin(0, 0)?;

    let lp_balance_after = logic.balance_of(&liquidity_provider);
    assert!(lp_balance_after < lp_balance);

    std::println!("✅ Remove liquidity one coin test passed");
    Ok(())
}

#[wasm_bindgen_test]
fn test_swap() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");
    let swapper = alkane_id("swapper");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(10);
    logic.set_admin_fee(1);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let context = Context {
        caller: swapper,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 100_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    let response = logic.swap(1, 0)?;

    let swapped_amount = response.alkanes.0[0].value;
    assert!(swapped_amount > 0);

    std::println!("   └─ Swapped Amount: {}", swapped_amount);
    std::println!("✅ Swap test passed");
    Ok(())
}

#[wasm_bindgen_test]
fn test_claim_admin_fees() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");
    let swapper = alkane_id("swapper");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(FEE_DENOMINATOR / 10);
    logic.set_admin_fee(FEE_DENOMINATOR / 2);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let context = Context {
        caller: swapper,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 100_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.swap(1, 0)?;

    let admin_balance_before = logic.admin_balances(1);
    assert!(admin_balance_before > U256::ZERO);

    let context = Context {
        caller: owner,
        ..Default::default()
    };

    logic.context = context;
    logic.claim_admin_fees()?;

    let admin_balance_after = logic.admin_balances(1);
    assert_eq!(admin_balance_after, U256::ZERO);

    std::println!("✅ Claim admin fees test passed");
    Ok(())
}

#[wasm_bindgen_test]
fn test_remove_liquidity() -> Result<()> {
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");
    let liquidity_provider = alkane_id("liquidity_provider");

    let mut logic = Logic::<MockStorage>::new();
    logic.set_coins(0, token_a);
    logic.set_coins(1, token_b);
    logic.set_A(U256::from(100));
    logic.set_fee(10);
    logic.set_admin_fee(1);
    logic.set_owner(owner);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: token_a,
                value: 1_000_000,
            },
            AlkaneTransfer {
                id: token_b,
                value: 1_000_000,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.add_liquidity(1000)?;

    let lp_balance = logic.balance_of(&liquidity_provider);
    assert!(lp_balance > 0);

    let context = Context {
        caller: liquidity_provider,
        incoming_alkanes: AlkaneTransferParcel(vec![
            AlkaneTransfer {
                id: logic.context.myself,
                value: lp_balance,
            },
        ]),
        ..Default::default()
    };

    logic.context = context;
    logic.remove_liquidity(vec![0, 0])?;

    let lp_balance = logic.balance_of(&liquidity_provider);
    assert_eq!(lp_balance, 0);

    std::println!("✅ Remove liquidity test passed");
    Ok(())
}
