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

use ruint::aliases::U256;
use anyhow::{Result, bail};

const N_COINS: U256 = U256::from_limbs([2, 0, 0, 0]);
const A_PRECISION: U256 = U256::from_limbs([100, 0, 0, 0]);

/// D invariant calculation in non-overflowing integer operations
/// iteratively
///
/// A * sum(x_i) * n**n + D = A * D * n**n + D**(n+1) / (n**n * prod(x_i))
///
/// Converging solution:
/// D[j+1] = (A * n**n * sum(x_i) - D[j]**(n+1) / (n**n prod(x_i))) / (A * n**n - 1)
pub fn get_D(xp: &[U256; 2], amp: U256) -> Result<U256> {
    let mut S = U256::ZERO;
    for &x in xp.iter() {
        S += x;
    }
    if S == U256::ZERO {
        return Ok(U256::ZERO);
    }

    let mut Dprev;
    let mut D = S;
    let Ann = amp * N_COINS;

    for _i in 0..255 {
        let mut D_P = D;
        for &x in xp.iter() {
            // D_P = D_P * D / (x * N_COINS + 1)
            D_P = D_P * D / (x.saturating_mul(N_COINS).saturating_add(U256::from(1)));
        }
        Dprev = D;
        // D = (Ann * S / A_PRECISION + D_P * N_COINS) * D / ((Ann - A_PRECISION) * D / A_PRECISION + (N_COINS + 1) * D_P)
        let num = (Ann.saturating_mul(S) / A_PRECISION + D_P.saturating_mul(N_COINS)).saturating_mul(D);
        let den = (Ann.saturating_sub(A_PRECISION)).saturating_mul(D) / A_PRECISION + (N_COINS + U256::from(1)).saturating_mul(D_P);
        D = num / den;
        if D > Dprev {
            if D - Dprev <= U256::from(1) {
                return Ok(D);
            }
        } else {
            if Dprev - D <= U256::from(1) {
                return Ok(D);
            }
        }
    }
    bail!("D does not converge");
}

/// Calculate x[j] if one makes x[i] = x
///
/// Done by solving quadratic equation iteratively.
/// x_1**2 + x1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
/// x_1**2 + b*x_1 = c
///
/// x_1 = (x_1**2 + c) / (2*x_1 + b)
pub fn get_y(i: usize, j: usize, x: U256, xp: &[U256; 2], amp: U256, D: U256) -> Result<U256> {
    // x in the input is converted to the same price/precision
    assert!(i != j);
    assert!(i < 2);
    assert!(j < 2);

    let Ann = amp * N_COINS;
    let mut c = D;
    let mut S_ = U256::ZERO;
    let mut _x;

    for _i in 0..N_COINS.as_limbs()[0] as usize {
        if _i == i {
            _x = x;
        } else if _i != j {
            _x = xp[_i];
        } else {
            continue;
        }
        S_ += _x;
        // c = c * D / (_x * N_COINS)
        c = c.saturating_mul(D) / (_x.saturating_mul(N_COINS));
    }
    // c = c * D * A_PRECISION / (Ann * N_COINS)
    c = c.saturating_mul(D).saturating_mul(A_PRECISION) / (Ann.saturating_mul(N_COINS));
    // b = S_ + D * A_PRECISION / Ann
    let b = S_ + D.saturating_mul(A_PRECISION) / Ann;
    let mut y_prev;
    let mut y = D;

    for _i in 0..255 {
        y_prev = y;
        // y = (y*y + c) / (2 * y + b - D)
        y = (y.saturating_mul(y) + c) / (U256::from(2).saturating_mul(y) + b - D);
        if y > y_prev {
            if y - y_prev <= U256::from(1) {
                return Ok(y);
            }
        } else {
            if y_prev - y <= U256::from(1) {
                return Ok(y);
            }
        }
    }

    bail!("y does not converge");
}

/// Calculate x[i] if one reduces D from being calculated for xp to D
///
/// Done by solving quadratic equation iteratively.
/// x_1**2 + x1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
/// x_1**2 + b*x_1 = c
///
/// x_1 = (x_1**2 + c) / (2*x_1 + b)
pub fn get_y_D(A: U256, i: usize, xp: &[U256; 2], D: U256) -> Result<U256> {
    assert!(i < 2);

    let Ann = A * N_COINS;
    let mut c = D;
    let mut S_ = U256::ZERO;
    let mut _x;

    for _i in 0..N_COINS.as_limbs()[0] as usize {
        if _i != i {
            _x = xp[_i];
        } else {
            continue;
        }
        S_ += _x;
        c = c * D / (_x * N_COINS);
    }
    c = c * D * A_PRECISION / (Ann * N_COINS);
    let b = S_ + D * A_PRECISION / Ann;
    let mut y_prev;
    let mut y = D;

    for _i in 0..255 {
        y_prev = y;
        y = (y * y + c) / (U256::from(2) * y + b - D);
        if y > y_prev {
            if y - y_prev <= U256::from(1) {
                return Ok(y);
            }
        } else {
            if y_prev - y <= U256::from(1) {
                return Ok(y);
            }
        }
    }

    bail!("y does not converge");
}
