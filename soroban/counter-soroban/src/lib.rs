#![no_std]
use soroban_sdk::{contract,contracterror, contractimpl, log, symbol_short, Env, Symbol};


// errors 
/**It must have the #[repr(u32)] attribute.
It must have the #[derive(Copy)] attribute.
Every variant must have an explicit integer value assigned. */

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}
const COUNTER : Symbol= symbol_short!("COUNTER");
const COUNTEREVENT : Symbol= symbol_short!("Increment");
#[contract]
pub struct CounterContract;

#[contractimpl]
impl CounterContract {
    pub fn increment(env: Env) -> u32{
        let mut counter:u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        
        counter +=1;
        // store the counter
        env.storage().instance().set(&COUNTER, &counter);
        //env.events().publish(topics, data);

        env.events().publish((COUNTER,&COUNTEREVENT), counter);
        log!(&env, "Hello {}", counter);
       counter
    }

    pub fn decrement(env: Env) -> u32{
     let mut counter:u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
     if counter > 0 {
        counter -=1;
        env.storage().instance().set(&COUNTER, &counter);
     }
     counter
    }
    pub fn get_counter(env: Env) -> u32{
env.storage().instance().get(&COUNTER).unwrap_or(0)
    }




//         //Errors can be returned from contract functions by returning Result<_, E>.
//    pub fn try_increment(env: Env) -> Result<u32, Error>{
//         let mut counter:u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
//         if counter >3 {
//             Err(Error::LimitReached)
 
//         }
//         else{
//  counter +=1;
//         // store the counter
//         env.storage().instance().set(&COUNTER, &counter);
//         //env.events().publish(topics, data);

//         env.events().publish((COUNTER,&COUNTEREVENT), counter);
//        Ok(counter)
//         }
       
//     }
//    pub fn try_increment_or_panic(env: Env) -> u32{
//         let mut counter:u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
//         if counter >3 {
//              panic_with_error!(&env, Error::LimitReached)

//         }
//         else{
//  counter +=1;
//         // store the counter
//         env.storage().instance().set(&COUNTER, &counter);
//         //env.events().publish(topics, data);

//         env.events().publish((COUNTER,&COUNTEREVENT), counter);
//        counter
//         }
       
 //   }
}

mod test;
