#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short,  Env, Symbol};

#[contract]
pub struct ContractA;
const VALUE: Symbol = symbol_short!("value");
#[contractimpl]
impl ContractA {
    pub fn set(env: Env, val: u32) {
         env.storage().instance().set(&VALUE,&val);
    }
    pub fn get(env: Env) ->u32 {
         env.storage().instance().get(&VALUE).unwrap_or(0)
    }
}

// mod test;
