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

use alkanes_runtime::{declare_alkane, message::MessageDispatch, runtime::AlkaneResponder, storage::StoragePointer};
use alkanes_support::{id::AlkaneId, response::CallResponse};
use anyhow::Result;
use metashrew_support::compat::{to_arraybuffer_layout, to_passback_ptr};
use ruint::aliases::U256;

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
        i: i128,
        min_amount: u128,
    },

    #[opcode(4)]
    RemoveLiquidityImbalance {
        amounts: Vec<u128>,
        max_burn_amount: u128,
    },

    #[opcode(5)]
    Swap {
        i: i128,
        j: i128,
        dx: u128,
        min_dy: u128,
    },

    #[opcode(10)]
    ClaimAdminFees,

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
    fn name(&self) -> String;
    fn symbol(&self) -> String;
}
impl MintableToken for SynthPool {
    fn name(&self) -> String {
        "æfrBTC-LP".to_string()
    }
    fn symbol(&self) -> String {
        "æfrBTC-LP".to_string()
    }
}

impl SynthPool {
    fn coins(&self, index: usize) -> AlkaneId {
        StoragePointer::new(&format!("/coins/{}", index)).get_or_default()
    }
    fn set_coins(&self, index: usize, value: AlkaneId) {
        StoragePointer::new(&format!("/coins/{}", index)).set(value)
    }
    fn A(&self) -> U256 {
        StoragePointer::from_keyword("/A").get_or_default()
    }
    fn set_A(&self, value: U256) {
        StoragePointer::from_keyword("/A").set(value)
    }
    fn fee(&self) -> u128 {
        StoragePointer::from_keyword("/fee").get_or_default()
    }
    fn set_fee(&self, value: u128) {
        StoragePointer::from_keyword("/fee").set(value)
    }
    fn admin_fee(&self) -> u128 {
        StoragePointer::from_keyword("/admin_fee").get_or_default()
    }
    fn set_admin_fee(&self, value: u128) {
        StoragePointer::from_keyword("/admin_fee").set(value)
    }
    fn balances(&self, index: usize) -> U256 {
        StoragePointer::new(&format!("/balances/{}", index)).get_or_default()
    }
    fn set_balances(&self, index: usize, value: U256) {
        StoragePointer::new(&format!("/balances/{}", index)).set(value)
    }
    fn admin_balances(&self, index: usize) -> U256 {
        StoragePointer::new(&format!("/admin_balances/{}", index)).get_or_default()
    }
    fn set_admin_balances(&self, index: usize, value: U256) {
        StoragePointer::new(&format!("/admin_balances/{}", index)).set(value)
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
            self.set_admin_balances(j, self.admin_balances(j) + dy_admin_fee);
        }

        self.set_balances(i, xp[i] + dx);
        self.set_balances(j, xp[j] - dy);

        Ok(dy)
    }

    fn _calc_withdraw_one_coin(&self, token_amount: U256, i: usize) -> Result<U256> {
        let amp = self.A();
        let xp = self._get_balances();
        let D0 = math::get_D(&xp, amp)?;
        let D1 = D0 - token_amount * D0 / self.total_supply();
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

    pub fn add_liquidity(
        &self,
        amounts: Vec<u128>,
        min_mint_amount: u128,
    ) -> Result<CallResponse> {
        let amp = self.A();
        let old_balances = self._get_balances();
        let token_supply = self.total_supply();
        let D0 = if token_supply > U256::ZERO {
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
        if token_supply > U256::ZERO {
            let mut fees = [U256::ZERO; 2];
            let n_coins = U256::from(N_COINS);
            let fee = U256::from(self.fee()) * n_coins / (U256::from(4) * (n_coins - U256::from(1)));
            let admin_fee = U256::from(self.admin_fee());

            for i in 0..N_COINS as usize {
                let ideal_balance = D1 * old_balances[i] / D0;
                let difference = if ideal_balance > new_balances[i] {
                    ideal_balance - new_balances[i]
                } else {
                    new_balances[i] - ideal_balance
                };
                fees[i] = fee * difference / U256::from(FEE_DENOMINATOR);
                self.set_admin_balances(
                    i,
                    self.admin_balances(i) + fees[i] * admin_fee / U256::from(FEE_DENOMINATOR),
                );
                new_balances[i] -= fees[i];
            }
            let D2 = math::get_D(&new_balances, amp)?;
            mint_amount = token_supply * (D2 - D0) / D0;
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

        for i in 0..N_COINS as usize {
            self.runtime().transfer_from(self.coins(i), self.runtime().sender(), amounts[i])?;
        }
        self.mint(self.runtime().sender(), mint_amount.try_into().unwrap())?;

        Ok(CallResponse::default())
    }

    pub fn remove_liquidity(
        &self,
        amount: u128,
        min_amounts: Vec<u128>,
    ) -> Result<CallResponse> {
        let total_supply = self.total_supply();
        let mut amounts = [U256::ZERO; 2];
        let balances = self._get_balances();
        let amount_u256 = U256::from(amount);

        for i in 0..N_COINS as usize {
            let value = balances[i] * amount_u256 / total_supply;
            anyhow::ensure!(
                value >= U256::from(min_amounts[i]),
                "Withdrawal resulted in fewer coins than expected"
            );
            amounts[i] = value;
            self.set_balances(i, balances[i] - value);
        }

        self.burn(self.runtime().sender(), amount)?;
        for i in 0..N_COINS as usize {
            self.runtime().transfer(
                self.coins(i),
                self.runtime().sender(),
                amounts[i].try_into().unwrap(),
            )?;
        }

        Ok(CallResponse::default())
    }

    pub fn remove_liquidity_imbalance(
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
        let fee = U256::from(self.fee()) * n_coins / (U256::from(4) * (n_coins - U256::from(1)));
        let admin_fee = U256::from(self.admin_fee());

        for i in 0..N_COINS as usize {
            let ideal_balance = D1 * old_balances[i] / D0;
            let difference = if ideal_balance > new_balances[i] {
                ideal_balance - new_balances[i]
            } else {
                new_balances[i] - ideal_balance
            };
            fees[i] = fee * difference / U256::from(FEE_DENOMINATOR);
            self.set_admin_balances(
                i,
                self.admin_balances(i) + fees[i] * admin_fee / U256::from(FEE_DENOMINATOR),
            );
            new_balances[i] -= fees[i];
        }

        let D2 = math::get_D(&new_balances, amp)?;
        let token_amount = token_supply * (D0 - D2) / D0;
        anyhow::ensure!(
            token_amount <= U256::from(max_burn_amount),
            "Slippage screwed you"
        );

        for i in 0..N_COINS as usize {
            self.set_balances(i, old_balances[i] - U256::from(amounts[i]));
        }

        self.burn(self.runtime().sender(), token_amount.try_into().unwrap())?;
        for i in 0..N_COINS as usize {
            self.runtime().transfer(
                self.coins(i),
                self.runtime().sender(),
                U256::from(amounts[i]).try_into().unwrap(),
            )?;
        }

        Ok(CallResponse::default())
    }

    pub fn remove_liquidity_one_coin(
        &self,
        token_amount: u128,
        i: i128,
        min_amount: u128,
    ) -> Result<CallResponse> {
        let i = i as usize;
        let token_amount_u256 = U256::from(token_amount);
        let min_amount_u256 = U256::from(min_amount);

        let dy = self._calc_withdraw_one_coin(token_amount_u256, i)?;
        anyhow::ensure!(dy >= min_amount_u256, "Not enough coins removed");

        self.set_balances(i, self.balances(i) - dy);

        self.burn(self.runtime().sender(), token_amount)?;
        self.runtime().transfer(self.coins(i), self.runtime().sender(), dy.try_into().unwrap())?;

        Ok(CallResponse::default())
    }

    pub fn swap(&self, i: i128, j: i128, dx: u128, min_dy: u128) -> Result<CallResponse> {
        let i = i as usize;
        let j = j as usize;
        let dx_u256 = U256::from(dx);
        let min_dy_u256 = U256::from(min_dy);

        let dy = self._exchange(i, j, dx_u256)?;
        anyhow::ensure!(dy >= min_dy_u256, "Slippage screwed you");

        self.runtime().transfer_from(self.coins(i), self.runtime().sender(), dx)?;
        self.runtime().transfer(self.coins(j), self.runtime().sender(), dy.try_into().unwrap())?;

        Ok(CallResponse::default())
    }

    pub fn claim_admin_fees(&self) -> Result<CallResponse> {
        let owner = self.owner();
        anyhow::ensure!(self.runtime().sender() == owner, "Not the owner");
        for i in 0..N_COINS as usize {
            let amount = self.admin_balances(i);
            if amount > U256::ZERO {
                self.set_admin_balances(i, U256::ZERO);
                self.runtime().transfer(
                    self.coins(i),
                    owner,
                    amount.try_into().unwrap(),
                )?;
            }
        }
        Ok(CallResponse::default())
    }

    pub fn get_virtual_price(&self) -> Result<CallResponse> {
        let balances = self._get_balances();
        let amp = self.A();
        let D = math::get_D(&balances, amp)?;
        let token_supply = self.total_supply();
        let virtual_price = D * U256::from(PRECISION) / token_supply;
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

    pub fn get_A(&self) -> Result<CallResponse> {
        let mut response = CallResponse::default();
        response.data = self.A().to_le_bytes_vec();
        Ok(response)
    }
}

declare_alkane!{impl AlkaneResponder for SynthPool {
  type Message = SynthPoolMessage;
}}

#[cfg(test)]
mod tests;
