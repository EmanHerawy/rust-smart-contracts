multiversx_sc::imports!();
multiversx_sc::derive_imports!();
#[multiversx_sc::contract]
pub trait HelloWorld {
     #[view(gethello)]
    #[storage_mapper("hello")]
    fn hello(&self) -> SingleValueMapper<ManagedBuffer>;

    #[init]
    fn init(&self, initial_value: ManagedBuffer) {
        self.hello().set(initial_value);
    }

    /// Add desired amount to the storage variable.
     #[endpoint]
    fn say_hello(&self, msg: ManagedBuffer) {
        self.hello().set(msg);
    }
}
