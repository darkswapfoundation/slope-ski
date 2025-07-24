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

use alkanes_runtime::{
    declare_alkane,
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
use ruint::aliases::U256;
use serde::{de::Visitor, de::MapAccess, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use std::ops::Deref;
use std::sync::Arc;
use std::fmt;

const N_COINS: u128 = 2;
const PRECISION: u128 = 10u128.pow(18); // 1e18
const FEE_DENOMINATOR: u128 = 10u128.pow(10);

// aeBTC and frBTC
const TOKEN_NAMES: [&str; 2] = ["æBTC", "frBTC"];

#[derive(MessageDispatch)]
pub enum SynthPoolMessage {
    #[opcode(0)]
    InitPool {
        // The two tokens in the pool
        token_a: AlkaneId,
        token_b: AlkaneId,
        // The amplification parameter
        A: u128,
        // The fee, scaled by 10^10
        fee: u128,
        // The admin fee, scaled by 10^10
        admin_fee: u128,
        // The owner of the contract
        owner: AlkaneId,
    },

    #[opcode(1)]
    AddLiquidity {
        amounts: Vec<u128>,
        min_mint_amount: u128,
    },

    #[opcode(2)]
    RemoveLiquidity {
        amount: u128,
        min_amounts: Vec<u128>,
    },

    #[opcode(3)]
    RemoveLiquidityOneCoin {
        token_amount: u128,
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
        i: u128,
        j: u128,
        dx: u128,
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

#[derive(Default)]
pub struct SynthPool();

pub trait MintableToken {
    fn total_supply(&self) -> u128;
    fn set_total_supply(&self, value: u128);
    fn balance_of(&self, owner: &AlkaneId) -> u128;
    fn set_balance_of(&self, owner: &AlkaneId, value: u128);
    fn mint(&self, to: &AlkaneId, amount: u128) -> Result<()>;
    fn burn(&self, from: &AlkaneId, amount: u128) -> Result<()>;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
}

pub trait OwnedToken {
    fn owner(&self) -> AlkaneId;
    fn set_owner(&self, owner: AlkaneId);
}

impl MintableToken for SynthPool {
    fn total_supply(&self) -> u128 {
        StoragePointer::from_keyword("/total_supply").get_value::<u128>()
    }
    fn set_total_supply(&self, value: u128) {
        StoragePointer::from_keyword("/total_supply").set_value::<u128>(value);
    }
    fn balance_of(&self, owner: &AlkaneId) -> u128 {
        StoragePointer::from_keyword("/balance/").select(&(*owner).into()).get_value::<u128>()
    }
    fn set_balance_of(&self, owner: &AlkaneId, value: u128) {
        StoragePointer::from_keyword("/balance/").select(&(*owner).into()).set_value::<u128>(value);
    }
    fn mint(&self, to: &AlkaneId, amount: u128) -> Result<()> {
        self.set_total_supply(self.total_supply() + amount);
        self.set_balance_of(to, self.balance_of(to) + amount);
        Ok(())
    }
    fn burn(&self, from: &AlkaneId, amount: u128) -> Result<()> {
        let balance = self.balance_of(from);
        anyhow::ensure!(balance >= amount, "Insufficient balance");
        self.set_balance_of(from, balance - amount);
        self.set_total_supply(self.total_supply() - amount);
        Ok(())
    }
    fn name(&self) -> String {
        "æfrBTC-LP".to_string()
    }
    fn symbol(&self) -> String {
        "æfrBTC-LP".to_string()
    }
}

impl OwnedToken for SynthPool {
    fn owner(&self) -> AlkaneId {
        let data = StoragePointer::from_keyword("/owner").get();
        if data.is_empty() {
            Default::default()
        } else {
            AlkaneId::try_from(data.as_slice().to_vec()).unwrap_or_default()
        }
    }
    fn set_owner(&self, owner: AlkaneId) {
        StoragePointer::from_keyword("/owner").set(Arc::new(owner.into()))
    }
}

impl SynthPool {
    fn coins(&self, index: usize) -> AlkaneId {
        let data = StoragePointer::from_keyword(&format!("/coins/{}", index)).get();
        if data.is_empty() {
            Default::default()
        } else {
            AlkaneId::try_from(data.as_slice().to_vec()).unwrap_or_default()
        }
    }
    fn set_coins(&self, index: usize, value: AlkaneId) {
        StoragePointer::from_keyword(&format!("/coins/{}", index)).set(Arc::new(value.into()))
    }
    fn A(&self) -> U256 {
        let data = StoragePointer::from_keyword("/A").get();
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_A(&self, value: U256) {
        StoragePointer::from_keyword("/A").set(Arc::new(value.to_le_bytes::<32>().to_vec()))
    }
    fn fee(&self) -> u128 {
        StoragePointer::from_keyword("/fee").get_value::<u128>()
    }
    fn set_fee(&self, value: u128) {
        StoragePointer::from_keyword("/fee").set_value::<u128>(value)
    }
    fn admin_fee(&self) -> u128 {
        StoragePointer::from_keyword("/admin_fee").get_value::<u128>()
    }
    fn set_admin_fee(&self, value: u128) {
        StoragePointer::from_keyword("/admin_fee").set_value::<u128>(value)
    }
    fn balances(&self, index: usize) -> U256 {
        let data = StoragePointer::from_keyword(&format!("/balances/{}", index)).get();
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_balances(&self, index: usize, value: U256) {
        StoragePointer::from_keyword(&format!("/balances/{}", index)).set(Arc::new(value.to_le_bytes::<32>().to_vec()))
    }
    fn admin_balances(&self, index: usize) -> U256 {
        let data = StoragePointer::from_keyword(&format!("/admin_balances/{}", index)).get();
        if data.is_empty() {
            U256::ZERO
        } else {
            U256::from_le_slice(&data)
        }
    }
    fn set_admin_balances(&self, index: usize, value: U256) {
        StoragePointer::from_keyword(&format!("/admin_balances/{}", index)).set(Arc::new(value.to_le_bytes::<32>().to_vec()))
    }

    fn _get_balances(&self) -> [U256; 2] {
        [self.balances(0), self.balances(1)]
    }

    fn _exchange(&self, i: usize, j: usize, dx: U256) -> Result<U256> {
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

    fn init_pool(
        &self,
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

    fn add_liquidity(
        &self,
        amounts: Vec<u128>,
        min_mint_amount: u128,
    ) -> Result<CallResponse> {
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
            "Slippage screwed you"
        );

        for i in 0..N_COINS as usize {
            self.set_balances(i, new_balances[i]);
        }

        let response = CallResponse::default();
        let context = self.context()?;
        self.mint(&context.caller, mint_amount.try_into().unwrap())?;

        Ok(response)
    }

    fn remove_liquidity(
        &self,
        amount: u128,
        min_amounts: Vec<u128>,
    ) -> Result<CallResponse> {
        let total_supply = self.total_supply();
        let mut amounts = [U256::ZERO; 2];
        let balances = self._get_balances();
        let amount_u256 = U256::from(amount);

        for i in 0..N_COINS as usize {
            let value = balances[i] * amount_u256 / U256::from(total_supply);
            anyhow::ensure!(
                value >= U256::from(min_amounts[i]),
                "Withdrawal resulted in fewer coins than expected"
            );
            amounts[i] = value;
            self.set_balances(i, balances[i] - value);
        }

        let context = self.context()?;
        self.burn(&context.caller, amount)?;
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

    fn remove_liquidity_imbalance(
        &self,
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
            "Slippage screwed you"
        );

        for i in 0..N_COINS as usize {
            self.set_balances(i, old_balances[i] - U256::from(amounts[i]));
        }

        let context = self.context()?;
        self.burn(&context.caller, token_amount.try_into().unwrap())?;
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

    fn remove_liquidity_one_coin(
        &self,
        token_amount: u128,
        i: u128,
        min_amount: u128,
    ) -> Result<CallResponse> {
        let i = i as usize;
        let token_amount_u256 = U256::from(token_amount);
        let min_amount_u256 = U256::from(min_amount);

        let dy = self._calc_withdraw_one_coin(token_amount_u256, i)?;
        anyhow::ensure!(dy >= min_amount_u256, "Not enough coins removed");

        let balance = self.balances(i);
        self.set_balances(i, balance - dy);

        let context = self.context()?;
        self.burn(&context.caller, token_amount)?;

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(vec![AlkaneTransfer {
                id: self.coins(i),
                value: dy.try_into().unwrap(),
            }]),
            ..Default::default()
        })
    }

    fn swap(
        &self,
        i: u128,
        j: u128,
        dx: u128,
        min_dy: u128,
    ) -> Result<CallResponse> {
        let i = i as usize;
        let j = j as usize;
        let dx_u256 = U256::from(dx);
        let min_dy_u256 = U256::from(min_dy);

        let dy = self._exchange(i, j, dx_u256)?;
        anyhow::ensure!(dy >= min_dy_u256, "Slippage screwed you");

        Ok(CallResponse {
            alkanes: AlkaneTransferParcel(vec![AlkaneTransfer {
                id: self.coins(j),
                value: dy.try_into().unwrap(),
            }]),
            ..Default::default()
        })
    }

    fn claim_admin_fees(&self) -> Result<CallResponse> {
        let context = self.context()?;
        let owner = self.owner();
        anyhow::ensure!(context.caller == owner, "Not the owner");
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

    fn get_virtual_price(&self) -> Result<CallResponse> {
        let balances = self._get_balances();
        let amp = self.A();
        let D = math::get_D(&balances, amp)?;
        let token_supply = self.total_supply();
        let virtual_price = D * U256::from(PRECISION) / U256::from(token_supply);
        let mut response = CallResponse::default();
        response.data = virtual_price.to_le_bytes_vec();
        Ok(response)
    }

    fn get_balances(&self) -> Result<CallResponse> {
        let balances = self._get_balances();
        let mut response = CallResponse::default();
        response.data.extend_from_slice(&balances[0].to_le_bytes_vec());
        response.data.extend_from_slice(&balances[1].to_le_bytes_vec());
        Ok(response)
    }

    fn get_a(&self) -> Result<CallResponse> {
        let mut response = CallResponse::default();
        response.data = self.A().to_le_bytes_vec();
        Ok(response)
    }

    fn forward(&self) -> Result<CallResponse> {
        Ok(CallResponse::default())
    }
}

impl AlkaneResponder for SynthPool {}

declare_alkane! {
    impl AlkaneResponder for SynthPool {
        type Message = SynthPoolMessage;
    }
}

#[cfg(test)]
mod tests;
