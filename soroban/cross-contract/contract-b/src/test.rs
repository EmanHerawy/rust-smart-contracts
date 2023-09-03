use crate::{ContractB, ContractBClient,contract_a};
use soroban_sdk::{  Env, IntoVal, FromVal, Val, Vec,symbol_short, Symbol};

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
let mut sig :Symbol=symbol_short!("set");
    let mut fn_args: Vec<Val> = (3u32,).into_val(&env);
    instance_b.a_by_invoke(&contract_a_id, &sig, &fn_args);
assert_eq!(instance_b.get_a(&contract_a_id),3);

 sig =symbol_short!("get");
      fn_args = ().into_val(&env);
   let value= instance_b.a_by_invoke(&contract_a_id, &sig, &fn_args);
// assert_eq!(instance_b.get_a(&contract_a_id),3);
assert_eq!(u32::from_val(&env,&value),3);
 
     std::println!("****************************************************************************");
 
    
}
