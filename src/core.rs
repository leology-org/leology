use core::str::FromStr;
use std::process::Command;

use rand::SeedableRng;
use snarkvm::console::{
    account::PrivateKey,
    network::Testnet3,
    prelude::{Environment, Uniform},
    types::Field,
};

use anyhow::{anyhow, Result};
use rand_chacha::ChaChaRng;

type Network = Testnet3;

use reqwest::{Client, Error};
use serde_json::Value;

const BASE_URL: &str = "http://localhost:3030";

async fn fetch_from_endpoint(endpoint: &str) -> Result<Value, Error> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = Client::new();
    client.get(url).send().await?.json::<Value>().await
}

// Function to get the latest block height
pub async fn get_block_height_latest() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/block/height/latest").await
}

// Function to get the latest block hash
pub async fn get_block_hash_latest() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/block/hash/latest").await
}

// Function to get the latest block
pub async fn get_block_latest() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/block/latest").await
}

// Function to get a block by height or hash
pub async fn get_block(height_or_hash: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/block/{}", height_or_hash)).await
}

// Function to get blocks in a range
pub async fn get_blocks(start_height: u32, end_height: u32) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/blocks?start={}&end={}", start_height, end_height)).await
}

// Function to get the height of a block by its hash
pub async fn get_height_by_hash(hash: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/height/{}", hash)).await
}

// Function to get transactions in a block by height
pub async fn get_block_transactions_by_height(height: u32) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/block/{}/transactions", height)).await
}

// Function to get a transaction by its ID
pub async fn get_transaction_by_id(tx_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/transaction/{}", tx_id)).await
}

// Function to get a confirmed transaction by its ID
pub async fn get_confirmed_transaction_by_id(tx_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/transaction/confirmed/{}", tx_id)).await
}

// Function to get memory pool transmissions
pub async fn get_memory_pool_transmissions() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/memoryPool/transmissions").await
}

// Function to get memory pool solutions
pub async fn get_memory_pool_solutions() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/memoryPool/solutions").await
}

// Function to get memory pool transactions
pub async fn get_memory_pool_transactions() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/memoryPool/transactions").await
}

// Function to get a program by its ID
pub async fn get_program_by_id(program_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/program/{}", program_id)).await
}

// Function to get mapping names of a program by its ID
pub async fn get_mapping_names_by_program_id(program_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/program/{}/mappings", program_id)).await
}

// Function to get the state path for a commitment
pub async fn get_state_path_for_commitment(commitment: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/statePath/{}", commitment)).await
}

// Function to get the latest state root
pub async fn get_state_root_latest() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/stateRoot/latest").await
}

// Function to get the latest committee
pub async fn get_committee_latest() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/committee/latest").await
}

// Function to get the count of peers
pub async fn get_peers_count() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/peers/count").await
}

// Function to get all peers
pub async fn get_peers_all() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/peers/all").await
}

// Function to get metrics of all peers
pub async fn get_peers_all_metrics() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/peers/all/metrics").await
}

// Function to get the node address
pub async fn get_node_address() -> Result<Value, Error> {
    fetch_from_endpoint("/testnet3/node/address").await
}

// Function to find the block hash by transaction ID
pub async fn find_block_hash_by_transaction_id(tx_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/find/blockHash/{}", tx_id)).await
}

// Function to find the transaction ID from a program ID
pub async fn find_transaction_id_from_program_id(program_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/find/transactionID/deployment/{}", program_id)).await
}

// Function to find the transaction ID from a transition ID
pub async fn find_transaction_id_from_transition_id(transition_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/find/transactionID/{}", transition_id)).await
}

// Function to find the transition ID from an input or output ID
pub async fn find_transition_id(input_or_output_id: &str) -> Result<Value, Error> {
    fetch_from_endpoint(&format!("/testnet3/find/transitionID/{}", input_or_output_id)).await
}

// Combined function to get and parse the mapping value
pub async fn get_and_parse_mapping_value<T: std::str::FromStr>(program_id: &str, mapping_name: &str, mapping_key: &str) -> Result<T, &'static str> 
where 
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let url = format!("{}/testnet3/program/{}/mapping/{}/{}", BASE_URL, program_id, mapping_name, mapping_key);
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|_| "Failed to send request")?.text().await.map_err(|_| "Failed to get response text")?;

    response.trim().parse::<T>().map_err(|_| "Failed to parse value")
}

// Example usage (assuming you're in an async context)
// let value: u64 = get_and_parse_mapping_value("program_id", "mapping_name", "mapping_key").await?;


pub fn devnet() -> std::io::Result<()> {
    // Execute the bash script using the Command module
    let status = Command::new("bash")
        .arg("devnet.sh")
        .status()?;

    // Check if the script executed successfully
    if status.success() {
        println!("Script executed successfully");
    } else {
        println!("Script execution failed");
    }

    Ok(())
}

pub fn new_account(seed: Option<String>) -> Result<snarkos_account::Account<Testnet3>> {
    // Recover the seed.
    let seed = match seed {
        // Recover the field element deterministically.
        Some(seed) => Field::new(
            <Network as Environment>::Field::from_str(&seed)
                .map_err(|e| anyhow!("Invalid seed - {e}"))?,
        ),
        // Sample a random field element.
        None => Field::rand(&mut ChaChaRng::from_entropy()),
    };
    // Recover the private key from the seed as a field element.
    let private_key = PrivateKey::try_from(seed)
        .map_err(|_| anyhow!("Failed to convert the seed into a valid private key"))?;
    // Construct the account.
    let account = snarkos_account::Account::<Network>::try_from(private_key)?;
    Ok(account)
}
