mod client_api;

use openai_api_rs::v1::api::Client;
use dotenv::dotenv;
use std::io::stdin;

fn main() {
    dotenv().ok();

    let client: Client = client_api::invoke();

    println!("Make your question for RChat: ");
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Your input is invalid.");

    client_api::request(client, input).expect("Error.");
}
