#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short,Address,  Env, Symbol};

mod contract_a {
    soroban_sdk::contractimport!(
        file = "../contract-a/target/wasm32-unknown-unknown/release/contract_a.wasm"
    );
}
#[contract]
pub struct ContractB;
const VALUE: Symbol = symbol_short!("value");
#[contractimpl]
impl ContractB {
    pub fn set_b(env: Env, val: u32) {
         env.storage().instance().set(&VALUE,&val);
    }

    pub fn get_b(env: Env) ->u32 {
         env.storage().instance().get(&VALUE).unwrap_or(0)

    }

    pub fn set_a(env: Env, val: u32, contract_address:Address) {
        let contract = contract_a::Client::new(&env, &contract_address);
         contract.set(&val);
    }
    pub fn get_a(env: Env,contract_address:Address) ->u32 {
        let contract = contract_a::Client::new(&env, &contract_address);

         contract.get()
    }
}

 mod test;
