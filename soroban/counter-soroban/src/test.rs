use crate::{CounterContract, CounterContractClient};
use soroban_sdk::{ symbol_short, Env, Symbol,testutils::Logs};

extern crate std;

#[test]
fn test_counter(){
let env = Env::default();
let contract_Id= env.register_contract(None,CounterContract);
let instance = CounterContractClient::new(&env,&contract_Id);
assert_eq!(instance.increment(),1);
instance.increment();
assert_eq!(instance.get_counter(),2);
assert_eq!(instance.decrement(),1);
 let logs = env.logger().all();
     std::println!("****************************************************************************");
     std::println!("{}", logs.join("\n"));

    
}
