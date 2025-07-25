//
// This file is part of the slope-ski project.
//
// The slope-ski project is free software: you can redistribute it and/or modify
// it under the terms of the MIT License.
//
// The slope-ski project is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// MIT License for more details.
//
// You should have received a copy of the MIT License
// along with the slope-ski project. If not, see <https://opensource.org/licenses/MIT>.
//
use super::{OwnedToken, SynthPool, SynthPoolMessage};
use alkanes::tests::{alkane_id, MockRuntime};
use anyhow::Result;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
pub fn test_initialize_pool() -> Result<()> {
    let mut runtime = MockRuntime::new();
    let token_a = alkane_id("token_a");
    let token_b = alkane_id("token_b");
    let owner = alkane_id("owner");

    let message = SynthPoolMessage::InitPool {
        token_a,
        token_b,
        A: 100,
        fee: 10,
        admin_fee: 1,
        owner,
    };

    runtime.dispatch(message, None)?;

    let pool = SynthPool::default();
    assert_eq!(pool.coins(0), token_a);
    assert_eq!(pool.coins(1), token_b);
    assert_eq!(pool.A().try_into().unwrap(), 100u128);
    assert_eq!(pool.fee(), 10);
    assert_eq!(pool.admin_fee(), 1);
    assert_eq!(pool.owner(), owner);

    Ok(())
}
