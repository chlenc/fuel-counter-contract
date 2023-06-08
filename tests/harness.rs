use std::env;

use dotenv::dotenv;
use fuels::prelude::*;

// Load abi from json
abigen!(Contract(
    name = "CounterContract",
    abi = "out/debug/counter-contract-abi.json"
));

const CONTRACT_ID: &str = "fuel18jh0gaal6yjhnyluflvxfpthzyks5p88g5uj2drwcure5quttnzs0m2958";
//"0x3caef477bfd1257993fc4fd8648577112d0a04e7453925346ec7079a038b5cc5";

#[tokio::test]
async fn do_increment() {
    dotenv().ok();
    let provider = Provider::connect("beta-3.fuel.network").await.unwrap();
    let wallet_pk = env::var("ADMIN").unwrap().parse().unwrap();
    let wallet = WalletUnlocked::new_from_private_key(wallet_pk, Some(provider.clone()));

    let contract_id: Bech32ContractId = CONTRACT_ID.parse().expect("Invalid ID");

    let instance = CounterContract::new(contract_id, wallet);
    let mathods = instance.methods();
    let params = TxParameters::default().set_gas_price(1);

    mathods.increment().tx_params(params).call().await.unwrap();
}
