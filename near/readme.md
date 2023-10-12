## note 
use stable version 
### build 
cargo build --target wasm32-unknown-unknown --release
### deploy 
near deploy --wasmFile ./target/wasm32-unknown-unknown/release/hello_world.wasm --accountId YOUR_ACCOUNT_HERE


you can check this contract via https://explorer.testnet.near.org/transactions/7NzENAfmuVtemFVWMdTP7NcwCcC8bw2SV8JEfPPue6tn