use crate::{HelloWorldContract, HelloWorldContractClient};
//Symbols are small efficient strings up to 32 characters in length and limited to a-z A-Z 0-9 _ that are encoded into 64-bit integers.
use soroban_sdk::{symbol_short, vec, Env};
/**Vec is a sequential and indexable growable collection type.
Values are stored in the environment and are available to contract through the functions defined on Vec. Values stored in the Vec are transmitted to the environment as RawVals, and when retrieved from the Vec are transmitted back and converted from RawVal back into their type. */
#[test]
fn hello() {
    /**In any test the first thing that is always required is an Env, which is the Soroban environment that the contract will run inside of. */
    let env = Env::default();
    //The contract is registered with the environment using the contract type. Contracts can specify a fixed contract ID as the first argument, or provide None and one will be generated.

    let contract_id = env.register_contract(None, HelloWorldContract);
    let client = HelloWorldContractClient::new(&env, &contract_id);
/**All public functions within an impl block that is annotated with the #[contractimpl] attribute have a corresponding function generated in a generated client type. The client type will be named the same as the contract type with Client appended. For example, in our contract the contract type is Contract, and the client is named ContractClient. */
    let words = client.say_hello(&symbol_short!("Dev"));
    //The values returned by functions can be asserted on:
    //  Dev -> to 
    assert_eq!(
        words,
        vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    );
}