use crate::{ContractB, ContractBClient,contract_a};
use soroban_sdk::{  Env, IntoVal , Val, Vec,symbol_short, Symbol};

extern crate std;

#[test]
fn test_cross_contraacts(){
let env = Env::default();

    // Register contract A using the imported WASM.
    let contract_a_id = env.register_contract_wasm(None, contract_a::WASM);

let contract_b_Id= env.register_contract(None,ContractB);
let instance_b = ContractBClient::new(&env,&contract_b_Id);

instance_b.set_b(&5);
instance_b.set_a(&2,&contract_a_id);
assert_eq!(instance_b.get_b(),5);
assert_eq!(instance_b.get_a(&contract_a_id),2);
// set contract a via invoke method
let sig :Symbol=symbol_short!("set");
    let fn_args: Vec<Val> = (3u32,).into_val(&env);
    instance_b.a_by_invoke(&contract_a_id, &sig, &fn_args);
assert_eq!(instance_b.get_a(&contract_a_id),3);
// instance.increment();
// assert_eq!(instance.get_counter(),2);
// assert_eq!(instance.decrement(),1);
//  let logs = env.logger().all();
     std::println!("****************************************************************************");
//      std::println!("{}", logs.join("\n"));

    
}
