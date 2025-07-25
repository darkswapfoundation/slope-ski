#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
/*
 * This file is part of the slope-ski project.
 *
 * The slope-ski project is free software: you can redistribute it and/or modify
 * it under the terms of the MIT License.
 *
 * The slope-ski project is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * MIT License for more details.
 *
 * You should have received a copy of the MIT License
 * along with the slope-ski project. If not, see <https://opensource.org/licenses/MIT>.
 */
mod math;

use std::fmt::Write;
use alkanes_runtime::{
    println,
    stdio::stdout,
    message::MessageDispatch,
    runtime::AlkaneResponder,
    storage::StoragePointer,
};
use alkanes_support::{
    context::Context,
    id::AlkaneId,
    parcel::{AlkaneTransfer, AlkaneTransferParcel},
    response::CallResponse,
};
use anyhow::{anyhow, Result};
use metashrew_support::{
    byte_view::ByteView,
    compat::{to_arraybuffer_layout, to_passback_ptr},
    index_pointer::KeyValuePointer,
};
pub use ruint::aliases::U256;
use serde::{de::Visitor, de::MapAccess, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::sync::Arc;
use std::fmt;
use std::collections::HashMap;

const N_COINS: u128 = 2;
const PRECISION: u128 = 10u128.pow(18); // 1e18
const FEE_DENOMINATOR: u128 = 10u128.pow(10);

// aeBTC and frBTC
const TOKEN_NAMES: [&str; 2] = ["æBTC", "frBTC"];

#[derive(MessageDispatch)]
pub enum SynthPoolMessage {
    #[opcode(0)]
    InitPool {
        token_a: AlkaneId,
        token_b: AlkaneId,
        A: u128,
        fee: u128,
        admin_fee: u128,
        owner: AlkaneId,
    },
    #[opcode(1)]
    AddLiquidity {
       min_mint_amount: u128,
   },
    #[opcode(2)]
    RemoveLiquidity {
       min_amounts: Vec<u128>,
   },
    #[opcode(3)]
    RemoveLiquidityOneCoin {
       i: u128,
       min_amount: u128,
   },
    #[opcode(4)]
    RemoveLiquidityImbalance {
        amounts: Vec<u128>,
        max_burn_amount: u128,
    },
    #[opcode(5)]
    Swap {
       j: u128,
       min_dy: u128,
   },
    #[opcode(10)]
    ClaimAdminFees,
    #[opcode(50)]
    Forward,
    #[opcode(100)]
    #[returns(u128)]
    GetVirtualPrice,
    #[opcode(101)]
    #[returns(u128, u128)]
    GetBalances,
    #[opcode(102)]
    #[returns(u128)]
    GetA,
}

pub trait Storage {
    fn get(&self, key: &Vec<u8>) -> Vec<u8>;
    fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>);
}

#[derive(Default)]
pub struct AlkaneStorage;
impl Storage for AlkaneStorage {
    fn get(&self, key: &Vec<u8>) -> Vec<u8> {
        StoragePointer::wrap(key).get().as_ref().clone()
    }
    fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) {
        StoragePointer::wrap(key).set(Arc::new(value.clone()));
    }
}

#[derive(Default)]
pub struct Logic<S: Storage> {
    storage: S,
    context: Context,
}

impl<S: Storage + Default> Logic<S> {
    pub fn new() -> Self {
        Self {
            storage: S::default(),
            context: Context::default(),
        }
    }
    
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }
}

pub trait MintableToken {
    fn total_supply(&self) -> u128;
    fn set_total_supply(&mut self, value: u128);
    fn balance_of(&self, owner: &AlkaneId) -> u128;
    fn set_balance_of(&mut self, owner: &AlkaneId, value: u128);
    fn mint(&mut self, to: &AlkaneId, amount: u128) -> Result<()>;
    fn burn(&mut self, from: &AlkaneId, amount: u128) -> Result<()>;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
}

pub trait OwnedToken {
    fn owner(&self) -> AlkaneId;
    fn set_owner(&mut self, owner: AlkaneId);
}

impl<S: Storage> MintableToken for Logic<S> {
    fn total_supply(&self) -> u128 {
        let data = self.storage.get(&b"/total_supply".to_vec());
        if data.is_empty() { 0 } else { u128::from_le_bytes(data.try_into().unwrap()) }
    }
    fn set_total_supply(&mut self, value: u128) {
        self.storage.set(&b"/total_supply".to_vec(), &value.to_le_bytes().to_vec());
    }
    fn balance_of(&self, owner: &AlkaneId) -> u128 {
        let key = StoragePointer::wrap(&b"/balance/".to_vec()).select(&(*owner).into()).unwrap().to_vec();
        let data = self.storage.get(&key);
        if data.is_empty() { 0 } else { u128::from_le_bytes(data.try_into().unwrap()) }
    }
    fn set_balance_of(&mut self, owner: &AlkaneId, value: u128) {
        let key = StoragePointer::wrap(&b"/balance/".to_vec()).select(&(*owner).into()).unwrap().to_vec();
        self.storage.set(&key, &value.to_le_bytes().to_vec());
    }
    fn mint(&mut self, to: &AlkaneId, amount: u128) -> Result<()> {
        let total_supply = self.total_supply();
        self.set_total_supply(total_supply + amount);
        let balance = self.balance_of(to);
        self.set_balance_of(to, balance + amount);
        Ok(())
    }
    fn burn(&mut self, from: &AlkaneId, amount: u128) -> Result<()> {
        let balance = self.balance_of(from);
        anyhow::ensure!(balance >= amount, "Insufficient balance");
        self.set_balance_of(from, balance - amount);
        let total_supply = self.total_supply();
        self.set_total_supply(total_supply - amount);
        Ok(())
    }
    fn name(&self) -> String {
        "æfrBTC-LP".to_string()
    }
    fn symbol(&self) -> String {
        "æfrBTC-LP".to_string()
    }
}

impl<S: Storage> OwnedToken for Logic<S> {
    fn owner(&self) -> AlkaneId {
        let data = self.storage.get(&b"/owner".to_vec());
        if data.is_empty() {
            Default::default()
        } else {
            AlkaneId::try_from(data.as_slice().to_vec()).unwrap_or_default()
        }
    }
    fn set_owner(&mut self, owner: AlkaneId) {
        self.storage.set(&b"/owner".to_vec(), &owner.into())
    }
}

impl<S: Storage + 'static> Logic<S> {
    pub fn coins(&self, index: usize) -> AlkaneId {
        let data = self.storage.get(&format!("/coins/{}", index).as_bytes().to_vec());
        if data.is_empty() {
            Default::default()
        } else {
            AlkaneId::try_from(data.as_slice().to_vec()).unwrap_or_default()
        }
    }
    fn set_coins(&mut self, index: usize, value: AlkaneId) {
        self.storage.set(&format!("/coins/{}", index).as_bytes().to_vec(), &value.into())
    }
    pub fn A(&self) -> U256 {
        let data = self.storage.get(&b"/A".to_vec());
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_A(&mut self, value: U256) {
        self.storage.set(&b"/A".to_vec(), &value.to_le_bytes::<32>().to_vec())
    }
    pub fn fee(&self) -> u128 {
        let data = self.storage.get(&b"/fee".to_vec());
        if data.is_empty() { 0 } else { u128::from_le_bytes(data.try_into().unwrap()) }
    }
    fn set_fee(&mut self, value: u128) {
        self.storage.set(&b"/fee".to_vec(), &value.to_le_bytes().to_vec())
    }
    pub fn admin_fee(&self) -> u128 {
        let data = self.storage.get(&b"/admin_fee".to_vec());
        if data.is_empty() { 0 } else { u128::from_le_bytes(data.try_into().unwrap()) }
    }
    fn set_admin_fee(&mut self, value: u128) {
        self.storage.set(&b"/admin_fee".to_vec(), &value.to_le_bytes().to_vec())
    }
    fn balances(&self, index: usize) -> U256 {
        let data = self.storage.get(&format!("/balances/{}", index).as_bytes().to_vec());
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_balances(&mut self, index: usize, value: U256) {
        self.storage.set(&format!("/balances/{}", index).as_bytes().to_vec(), &value.to_le_bytes::<32>().to_vec())
    }
    fn admin_balances(&self, index: usize) -> U256 {
        let data = self.storage.get(&format!("/admin_balances/{}", index).as_bytes().to_vec());
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_admin_balances(&mut self, index: usize, value: U256) {
        self.storage.set(&format!("/admin_balances/{}", index).as_bytes().to_vec(), &value.to_le_bytes::<32>().to_vec())
    }

    fn _get_balances(&self) -> [U256; 2] {
        [self.balances(0), self.balances(1)]
    }

    fn _burn_from_context(&mut self) -> Result<U256> {
        let context = self.context.clone();
        let amount = context.incoming_alkanes.0.iter().find(|v| v.id == context.myself).map_or(0, |v| v.value);
        anyhow::ensure!(amount > 0, "No LP tokens to burn in incoming transaction");
        self.burn(&context.caller, amount)?;
        Ok(U256::from(amount))
    }

    fn _exchange(&mut self, i: usize, j: usize, dx: U256) -> Result<U256> {
        let balances = self._get_balances();
        let xp = balances;
        let x = xp[i] + dx;
        let amp = self.A();
        let D = math::get_D(&xp, amp)?;
        let y = math::get_y(i, j, x, &xp, amp, D)?;

        let dy = xp[j] - y;
        let fee = U256::from(self.fee());
        let dy_fee = dy * fee / U256::from(FEE_DENOMINATOR);
        let dy = dy - dy_fee;

        let admin_fee = U256::from(self.admin_fee());
        if admin_fee > U256::ZERO {
            let dy_admin_fee = dy_fee * admin_fee / U256::from(FEE_DENOMINATOR);
            let admin_balances = self.admin_balances(j);
            self.set_admin_balances(j, admin_balances + dy_admin_fee);
        }

        self.set_balances(i, xp[i] + dx);
        self.set_balances(j, xp[j] - dy);

        Ok(dy)
    }

    fn _calc_withdraw_one_coin(&self, token_amount: U256, i: usize) -> Result<U256> {
        let amp = self.A();
        let xp = self._get_balances();
        let D0 = math::get_D(&xp, amp)?;
        let D1 = D0 - token_amount * D0 / U256::from(self.total_supply());
        let new_y = math::get_y_D(amp, i, &xp, D1)?;

        let mut xp_reduced = xp;
        let fee = U256::from(self.fee());
        for j in 0..N_COINS as usize {
            let dx_expected = if j == i {
                xp[j] * D1 / D0 - new_y
            } else {
                xp[j] - xp[j] * D1 / D0
            };
            xp_reduced[j] -= fee * dx_expected / U256::from(FEE_DENOMINATOR);
        }

        let dy = xp_reduced[i] - math::get_y_D(amp, i, &xp_reduced, D1)?;
        Ok(dy - U256::from(1))
    }
    pub fn init_pool(
        &mut self,
        token_a: AlkaneId,
        token_b: AlkaneId,
        A: u128,
        fee: u128,
        admin_fee: u128,
        owner: AlkaneId,
    ) -> Result<CallResponse> {
        self.set_coins(0, token_a);
        self.set_coins(1, token_b);
        self.set_A(U256::from(A));
        self.set_fee(fee);
        self.set_admin_fee(admin_fee);
        self.set_owner(owner);
        Ok(CallResponse::default())
    }

    pub fn add_liquidity(
        &mut self,
        min_mint_amount: u128,
    ) -> Result<CallResponse> {
        alkanes_runtime::println!("Adding liquidity with min_mint_amount: {}", min_mint_amount);
        let mut amounts = [0u128; N_COINS as usize];
        let coin0 = self.coins(0);
        let coin1 = self.coins(1);
        for transfer in self.context.incoming_alkanes.0.iter() {
            if transfer.id == coin0 {
                amounts[0] = transfer.value;
            } else if transfer.id == coin1 {
                amounts[1] = transfer.value;
            }
        }
        let amp = self.A();
        let old_balances = self._get_balances();
        let token_supply = self.total_supply();
        let D0 = if token_supply > 0 {
            math::get_D(&old_balances, amp)?
        } else {
            U256::ZERO
        };

        let mut new_balances = old_balances;
        for i in 0..N_COINS as usize {
            new_balances[i] += U256::from(amounts[i]);
        }

        let D1 = math::get_D(&new_balances, amp)?;
        anyhow::ensure!(D1 > D0, "D1 must be greater than D0");

        let mint_amount;
        if token_supply > 0 {
            let mut fees = [U256::ZERO; 2];
            let n_coins = U256::from(N_COINS);
            let fee =
                U256::from(self.fee()) * n_coins / (U256::from(4) * (n_coins - U256::from(1)));
            let admin_fee = U256::from(self.admin_fee());

            for i in 0..N_COINS as usize {
                let ideal_balance = D1 * old_balances[i] / D0;
                let difference = if ideal_balance > new_balances[i] {
                    ideal_balance - new_balances[i]
                } else {
                    new_balances[i] - ideal_balance
                };
                fees[i] = fee * difference / U256::from(FEE_DENOMINATOR);
                let admin_balances = self.admin_balances(i);
                self.set_admin_balances(
                    i,
                    admin_balances + fees[i] * admin_fee / U256::from(FEE_DENOMINATOR),
                );
                new_balances[i] -= fees[i];
            }
            let D2 = math::get_D(&new_balances, amp)?;
            mint_amount = U256::from(token_supply) * (D2 - D0) / D0;
        } else {
            mint_amount = D1;
        }

        anyhow::ensure!(
            mint_amount >= U256::from(min_mint_amount),
            "!slippage"
        );

        for i in 0..N_COINS as usize {
            self.set_balances(i, new_balances[i]);
        }

        let response = CallResponse::default();
        let context = self.context.clone();
        self.mint(&context.caller, mint_amount.try_into().unwrap())?;

        Ok(response)
    }

    pub fn remove_liquidity(
        &mut self,
        min_amounts: Vec<u128>,
    ) -> Result<CallResponse> {
        let total_supply = self.total_supply();
        let mut amounts = [U256::ZERO; 2];
        let balances = self._get_balances();
        let amount_u256 = self._burn_from_context()?;

        for i in 0..N_COINS as usize {
            let value = balances[i] * amount_u256 / U256::from(total_supply);
            anyhow::ensure!(
                value >= U256::from(min_amounts[i]),
                "Withdrawal resulted in fewer coins than expected"
            );
            amounts[i] = value;
            self.set_balances(i, balances[i] - value);
        }

        let mut outgoing_alkanes = vec![];
        for i in 0..N_COINS as usize {
            outgoing_alkanes.push(AlkaneTransfer {
                id: self.coins(i),
                value: amounts[i].try_into().unwrap(),
            });
        }

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(outgoing_alkanes),
            ..Default::default()
        })
    }

    pub fn remove_liquidity_imbalance(
        &mut self,
        amounts: Vec<u128>,
        max_burn_amount: u128,
    ) -> Result<CallResponse> {
        let amp = self.A();
        let old_balances = self._get_balances();
        let token_supply = self.total_supply();
        let D0 = math::get_D(&old_balances, amp)?;

        let mut new_balances = old_balances;
        for i in 0..N_COINS as usize {
            new_balances[i] -= U256::from(amounts[i]);
        }

        let D1 = math::get_D(&new_balances, amp)?;
        let mut fees = [U256::ZERO; 2];
        let n_coins = U256::from(N_COINS);
        let fee =
            U256::from(self.fee()) * n_coins / (U256::from(4) * (n_coins - U256::from(1)));
        let admin_fee = U256::from(self.admin_fee());

        for i in 0..N_COINS as usize {
            let ideal_balance = D1 * old_balances[i] / D0;
            let difference = if ideal_balance > new_balances[i] {
                ideal_balance - new_balances[i]
            } else {
                new_balances[i] - ideal_balance
            };
            fees[i] = fee * difference / U256::from(FEE_DENOMINATOR);
            let admin_balance = self.admin_balances(i);
            self.set_admin_balances(
                i,
                admin_balance + fees[i] * admin_fee / U256::from(FEE_DENOMINATOR),
            );
            new_balances[i] -= fees[i];
        }

        let D2 = math::get_D(&new_balances, amp)?;
        let token_amount = U256::from(token_supply) * (D0 - D2) / D0;
        anyhow::ensure!(
            token_amount <= U256::from(max_burn_amount),
            "!slippage"
        );

        for i in 0..N_COINS as usize {
            self.set_balances(i, old_balances[i] - U256::from(amounts[i]));
        }

        let caller = self.context.caller.clone();
        self.burn(&caller, token_amount.try_into().unwrap())?;
        let mut outgoing_alkanes = vec![];
        for i in 0..N_COINS as usize {
            outgoing_alkanes.push(AlkaneTransfer {
                id: self.coins(i),
                value: amounts[i].try_into().unwrap(),
            });
        }

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(outgoing_alkanes),
            ..Default::default()
        })
    }

    pub fn remove_liquidity_one_coin(
        &mut self,
        i: u128,
        min_amount: u128,
    ) -> Result<CallResponse> {
        let i_usize = i as usize;
        let token_amount_u256 = self._burn_from_context()?;
        let min_amount_u256 = U256::from(min_amount);

        let dy = self._calc_withdraw_one_coin(token_amount_u256, i_usize)?;
        anyhow::ensure!(dy >= min_amount_u256, "Not enough coins removed");

        let balance = self.balances(i_usize);
        self.set_balances(i_usize, balance - dy);

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(vec![AlkaneTransfer {
                id: self.coins(i_usize),
                value: dy.try_into().unwrap(),
            }]),
            ..Default::default()
        })
    }

    pub fn swap(
        &mut self,
        j: u128,
        min_dy: u128,
    ) -> Result<CallResponse> {
        let j_usize = j as usize;
        let coin0 = self.coins(0);
        let coin1 = self.coins(1);
        let mut incoming_transfer = None;
        for transfer in self.context.incoming_alkanes.0.iter() {
            if transfer.id == coin0 || transfer.id == coin1 {
                anyhow::ensure!(incoming_transfer.is_none(), "Cannot swap more than one coin at a time");
                incoming_transfer = Some(transfer);
            }
        }
        let transfer = incoming_transfer.ok_or_else(|| anyhow!("No coin to swap provided in transaction"))?;
        let i = if transfer.id == coin0 { 0 } else { 1 };
        anyhow::ensure!(i != j_usize, "Cannot swap a coin for itself");

        let dx_u256 = U256::from(transfer.value);
        let min_dy_u256 = U256::from(min_dy);

        let dy = self._exchange(i, j_usize, dx_u256)?;
        anyhow::ensure!(dy >= min_dy_u256, "Slippage screwed you");

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(vec![AlkaneTransfer {
                id: self.coins(j_usize),
                value: dy.try_into().unwrap(),
            }]),
            ..Default::default()
        })
    }

    pub fn claim_admin_fees(&mut self) -> Result<CallResponse> {
        let owner = self.owner();
        anyhow::ensure!(self.context.caller == owner, "Not the owner");
        let mut outgoing_alkanes = vec![];
        for i in 0..N_COINS as usize {
            let amount = self.admin_balances(i);
            if amount > U256::ZERO {
                self.set_admin_balances(i, U256::ZERO);
                outgoing_alkanes.push(AlkaneTransfer {
                    id: self.coins(i),
                    value: amount.try_into().unwrap(),
                });
            }
        }
        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(outgoing_alkanes),
            ..Default::default()
        })
    }

    pub fn get_virtual_price(&self) -> Result<CallResponse> {
        let balances = self._get_balances();
        let amp = self.A();
        let D = math::get_D(&balances, amp)?;
        let token_supply = self.total_supply();
        let virtual_price = D * U256::from(PRECISION) / U256::from(token_supply);
        let mut response = CallResponse::default();
        response.data = virtual_price.to_le_bytes_vec();
        Ok(response)
    }

    pub fn get_balances(&self) -> Result<CallResponse> {
        let balances = self._get_balances();
        let mut response = CallResponse::default();
        response.data.extend_from_slice(&balances[0].to_le_bytes_vec());
        response.data.extend_from_slice(&balances[1].to_le_bytes_vec());
        Ok(response)
    }

    pub fn get_a(&self) -> Result<CallResponse> {
        let mut response = CallResponse::default();
        response.data = self.A().to_le_bytes_vec();
        Ok(response)
    }

    pub fn forward(&self) -> Result<CallResponse> {
        Ok(CallResponse::default())
    }
}

#[derive(Default)]
pub struct SynthPool(Logic<AlkaneStorage>);

impl std::ops::Deref for SynthPool {
    type Target = Logic<AlkaneStorage>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SynthPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AlkaneResponder for SynthPool {
     fn context(&self) -> Result<Context> {
        Ok(self.0.context.clone())
    }
    fn set_context(&mut self, context: Context) {
        self.0.context = context;
    }
}

use slope_macros::declare_alkane;

declare_alkane! {
    impl AlkaneResponder for SynthPool {
        type Message = SynthPoolMessage;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct MockStorage {
        db: HashMap<Vec<u8>, Vec<u8>>,
    }

    impl Storage for MockStorage {
        fn get(&self, key: &Vec<u8>) -> Vec<u8> {
            self.db.get(key).cloned().unwrap_or_default()
        }
        fn set(&mut self, key: &Vec<u8>, value: &Vec<u8>) {
            self.db.insert(key.clone(), value.clone());
        }
    }

    mod tests;
}
