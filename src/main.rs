use near::{types::Data, Contract, NetworkConfig};
use near_account_id::AccountId;
// use rocket::tokio::task;
// use near_workspaces;
use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Welcome from fly.io!!!!!"
}

#[get("/get_proposal_ids")]
async fn get_proposal_ids() -> Result<String, rocket::http::Status> {
    let mainnet = near_workspaces::mainnet()
        .await
        .map_err(|_e| rocket::http::Status::InternalServerError)?;
    let account_id = "devhub.near".parse::<AccountId>().unwrap();
    let network = NetworkConfig::from(mainnet);
    let contract = Contract(account_id);

    // Let's fetch current value on a contract
    let result: Result<Data<Vec<i32>>, _> = contract
        // Please note that you can add any argument as long as it is deserializable by serde :)
        // feel free to use serde_json::json macro as well
        .call_function("get_all_proposal_ids", ())
        .unwrap()
        .read_only()
        .fetch_from(&network)
        .await;

    match result {
        Ok(current_value) => {
            println!("Current value: {:?}", current_value);
            Ok(format!("Hello, {:?}!", current_value))
        }
        Err(e) => {
            println!("Error fetching proposal ids: {:?}", e);
            Err(rocket::http::Status::InternalServerError)
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_proposal_ids])
}
