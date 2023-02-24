use ethers_core::{types::H256, utils::keccak256};
use ethers_providers::{Middleware, Provider};

use magi::derive::{stages::batches::RawTransaction, Pipeline};

#[tokio::test(flavor = "multi_thread")]
async fn test_attributes_match() {
    let start_epoch = 8494058;
    let start_block = 5503464;
    let num = 100;

    let mut pipeline = Pipeline::new(start_epoch);

    let mut i = 0;
    while i < num {
        if let Some(payload) = pipeline.next() {
            let hashes = get_tx_hashes(&payload.unwrap().transactions);
            let expected_hashes = get_expected_hashes(start_block + i).await;

            assert_eq!(hashes, expected_hashes);

            i += 1;
        }
    }
}

async fn get_expected_hashes(block_num: u64) -> Vec<H256> {
    let provider =
        Provider::try_from("https://opt-goerli.g.alchemy.com/v2/Olu7jiUDhtHf1iWldKzbBXGB6ImGs0XM")
            .unwrap();

    provider
        .get_block(block_num)
        .await
        .unwrap()
        .unwrap()
        .transactions
}

fn get_tx_hashes(txs: &[RawTransaction]) -> Vec<H256> {
    txs.iter()
        .map(|tx| H256::from_slice(&keccak256(&tx.0)))
        .collect()
}