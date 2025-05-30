use std::env;

use solana_stream_sdk::{CommitmentLevel, ShredstreamClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Get the shreds endpoint from environment variable
    let endpoint = env::var("SHREDS_ENDPOINT")
        .unwrap_or_else(|_| "https://shreds-ams-9.erpc.global".to_string());

    let mut client = ShredstreamClient::connect(&endpoint).await?;

    let request = ShredstreamClient::create_entries_request_for_accounts(
        vec!["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string()],
        vec![],
        vec![],
        Some(CommitmentLevel::Processed),
    );

    let mut stream = client.subscribe_entries(request).await?;

    while let Some(slot_entry) = stream.message().await? {
        let entries =
            match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                Ok(e) => e,
                Err(e) => {
                    println!("Deserialization failed with err: {e}");
                    continue;
                }
            };
        let transactions = entries
            .iter()
            .flat_map(|e| e.transactions.iter())
            .collect::<Vec<_>>();
        println!("transactions: {:?}", transactions[0]);
        println!(
            "slot {}, entries: {}, transactions: {}",
            slot_entry.slot,
            entries.len(),
            entries.iter().map(|e| e.transactions.len()).sum::<usize>()
        );
    }
    Ok(())
}
