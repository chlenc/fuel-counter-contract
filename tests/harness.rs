use dotenv::dotenv;
use fuels::prelude::*;
use std::env;
use std::str::FromStr;

// Load abi from json
abigen!(Contract(
    name = "CounterContract",
    abi = "out/debug/counter-contract-abi.json"
));

const COUNTER_CONTRACT_ADDRESS: &str =
    "0xece203bb05af7c491a00dfed979a5c3a8d09e7f3d1469122c1dbb17616f6bb61";

#[tokio::test]
async fn do_increment() {
    dotenv().ok();
    let provider = Provider::connect("beta-4.fuel.network").await.unwrap();
    let wallet_pk = env::var("ADMIN").unwrap().parse().unwrap();
    let wallet = WalletUnlocked::new_from_private_key(wallet_pk, Some(provider.clone()));

    let id = ContractId::from_str(COUNTER_CONTRACT_ADDRESS).unwrap();
    let instance = CounterContract::new(id, wallet);
    let mathods = instance.methods();
    let params = TxParameters::default()
        .set_gas_price(1)
        .set_gas_limit(10_000_000);

    mathods.increment().tx_params(params).call().await.unwrap();
}
