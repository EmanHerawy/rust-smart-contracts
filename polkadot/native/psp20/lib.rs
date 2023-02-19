
#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;
 // a great resource to depend on https://use.ink/ink-vs-solidity/ 
#[ink::contract]
mod psp20 {
    // you need to addd ink_prelude to crgo.toml to let it compile , by default it's not added  ink_prelude = { version = "3.3", default-features = false }

       use ink_prelude::string::String;
       // to use mapping , you need to import it and after the new updates contract must drive `SpreadAllocate` to compile
       use ink_storage::{
        traits::{
            SpreadAllocate,
            // SpreadLayout,
        },
        Mapping,
    };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct MyContract {
        /// Assign a balance to every account ID
        balances: Mapping<AccountId,Balance>,
        allowances: Mapping<(AccountId,AccountId),Balance>,
        total_supply:Balance,
        max_supply:Balance,
        admin:AccountId,
        symbol:String,
        name:String,
    }
 #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }
 #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }
    impl MyContract {
        /// Constructor to initialize the contract with an empty mapping.
        #[ink(constructor, payable)]
        pub fn new(max_supply_:Balance, name_:String,symbol_:String) -> Self {
            //required for mappings
            // this is to init with empty value
            // ink_lang::utils::initialize_contract(|_| {})
            // to init with valus 
            ink_lang::utils::initialize_contract(|contract: &mut Self| {

                contract.max_supply=max_supply_;
                contract.symbol=symbol_;
                contract.name=name_;
                contract.admin=Self::env().caller();
            })
        }


        /// Retrieve the balance of the caller.
        #[ink(message)]
        pub fn balance_of(&self, owner:AccountId) -> Option<Balance> {
            self.balances.get(owner)
        }
        /// Retrieve the allowance.
        #[ink(message)]
        pub fn allowance(&self, from:AccountId, to:AccountId) -> Option<Balance> {
            self.allowances.get((from,to))
        }
      /// Returns the token name.
       #[ink(message)]
       pub fn name(&self) -> String {
           self.name.clone()
       }
      /// Returns the token symbole.
       #[ink(message)]
       pub fn symbol(&self) -> String {
           self.symbol.clone()
       }

        /// Credit more money to the contract.
        #[ink(message, payable)]
        pub fn transfer(&mut self, to:AccountId,amount:Balance) {
            let caller = self.env().caller();
            self._transfer(caller,to,amount);
        }

        /// Withdraw all your balance from the contract.
        pub fn withdraw(&mut self) {
            let caller = self.env().caller();
            assert_eq!(caller, self.admin,"UnAuthorized" );
            let balance = Self::env().balance();
            self.env().transfer(caller, balance).unwrap()
        }

        // private func
     fn _mint(&mut self, to:AccountId, amount:Balance) {
             let balance = self.balances.get(to).unwrap();
             self.total_supply+=&amount;
             self.balances.insert(to, &(balance + amount));
             // emit event
               Self::env().emit_event(Transfer{
                from:self.env().account_id(), // temp use contract address unitl i could figure out how to set zero address
                to:to,
                value:amount
            });
        }   
fn _transfer(&mut self, from:AccountId,to:AccountId,amount:Balance) {
            // unwrap_or to handle null value
            let balance = self.balances.get(from).unwrap_or(0);
            let to_balance = self.balances.get(to).unwrap_or(0);
          assert!(balance>amount,"Insufficient fund" );
            self.balances.insert(from, &(balance -amount));
            self.balances.insert(to, &(to_balance +amount));
            // emit transfer event
            Self::env().emit_event(Transfer{
                from:from,
                to:to,
                value:amount
            });
        }

        // public func

  #[ink(message, payable)]
        pub fn mint(&mut self) {
            // check max supply
            let caller = self.env().caller();
            // TODO: check mint price
            let endowment = self.env().transferred_value();
            assert!(self.max_supply>= (self.total_supply+endowment),"");

            self._mint(caller, endowment);
        }


          #[ink(message)]
        pub fn transfer_from(&mut self, from:AccountId,to:AccountId,amount:Balance) {
            let allowance = self.allowances.get((Self::env().caller(),from)).unwrap_or(0);
            assert!(allowance>=amount,"Insufficient fund");
            self._transfer(from,to,amount);
        }
          #[ink(message)]
        pub fn approve(&mut self, spender:AccountId,value:Balance) {
            let owner=Self::env().caller();
           self.allowances.insert((owner,spender),&value);
           Self::env().emit_event(Approval{
            spender,owner,value
           })
        }

    }
}
