use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
/// Represents a wallet on the blockchain.
pub struct Account {
    /// The wallet address is the base58 check-encoded public key of
    /// the wallet.
    pub address: String,
    /// The latest balance of the wallet known to the API
    #[serde(deserialize_with = "Hnt::deserialize")]
    pub balance: Hnt,
    /// The data credit balance of the wallet known to the API
    pub dc_balance: u64,
    /// The security token balance of the wallet known to the API
    #[serde(deserialize_with = "Hst::deserialize")]
    pub sec_balance: Hst,
    /// The current nonce for the account
    pub nonce: u64,
    /// The speculative nonce for the account
    #[serde(default)]
    pub speculative_nonce: u64,
    /// The speculative security nonce for the account
    #[serde(default)]
    pub speculative_sec_nonce: u64,
}

/// Get all known accounts
pub fn all(client: &Client) -> Stream<Account> {
    client.fetch_stream("/accounts", NO_QUERY)
}

/// Get a specific account by its address
pub async fn get(client: &Client, address: &str) -> Result<Account> {
    client
        .fetch(&format!("/accounts/{}", address), NO_QUERY)
        .await
}

/// Get all hotspots owned by a given account
pub fn hotspots(client: &Client, address: &str) -> Stream<hotspots::Hotspot> {
    client.fetch_stream(&format!("/accounts/{}/hotspots", address), NO_QUERY)
}

/// Get all OUIs owned by a given account
pub fn ouis(client: &Client, address: &str) -> Stream<ouis::Oui> {
    client.fetch_stream(&format!("/accounts/{}/ouis", address), NO_QUERY)
}

/// Get all validators owned by a given account
pub fn validators(client: &Client, address: &str) -> Stream<validators::Validator> {
    client.fetch_stream(&format!("/accounts/{}/validators", address), NO_QUERY)
}

/// Get all the transactions for the account
#[cfg(feature = "transactions")]
pub fn transactions(client: &Client, address: &str) -> Stream<transactions::Transaction> {
    client.fetch_stream(&format!("/accounts/{}/activity", address), NO_QUERY)
}

/// Get all the transactions for the account
#[cfg(feature = "transactions")]
pub fn get_rewards_last(client: &Client, address: &str, duration: ChronoDuration) -> Stream<reward::Reward> {
    let max_time: DateTime<Utc> = Utc::now();
    let min_time= max_time - duration;
    let query = [
        ["max_time".to_string(), format!("{:?}", max_time)],
        ["min_time".to_string(), format!("{:?}", min_time)],
    ];

    client
        .fetch_stream(&format!("/accounts/{}/rewards", address), &query)

}

/// Get all the transactions for the account
#[cfg(feature = "transactions")]
pub fn get_rewards_since(client: &Client, address: &str, min_time: DateTime<Utc>) -> Stream<reward::Reward> {
    let max_time: DateTime<Utc> = Utc::now();
    let query = [
        ["max_time".to_string(), format!("{:?}", max_time)],
        ["min_time".to_string(), format!("{:?}", min_time)],
    ];

    client
        .fetch_stream(&format!("/accounts/{}/rewards", address), &query)

}

/// Get all the transactions for the account
#[cfg(feature = "transactions")]
pub fn get_rewards_between(client: &Client, address: &str, min_time: DateTime<Utc>, max_time: DateTime<Utc>) -> Stream<reward::Reward> {
    let query = [
        ["max_time".to_string(), format!("{:?}", max_time)],
        ["min_time".to_string(), format!("{:?}", min_time)],
    ];

    client
        .fetch_stream(&format!("/accounts/{}/rewards", address), &query)

}

/// Get a list of of up to a limit (maximum 1000) accounts sorted by their balance in
/// descending order
pub async fn richest(client: &Client, limit: Option<u32>) -> Result<Vec<Account>> {
    client
        .fetch(
            &format!("/accounts/rich?limit={}", limit.unwrap_or(1000)),
            NO_QUERY,
        )
        .await
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::test;

    #[test]
    async fn all() {
        let client = Client::default();
        let accounts =
            accounts::all(&client)
                .take(10)
                .fold(vec![], |mut acc, account| async move {
                    acc.push(account.unwrap().address);
                    acc
                });
        assert_eq!(accounts.await.len(), 10);
    }

    #[test]
    async fn get() {
        let client = Client::default();
        let account = accounts::get(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
        .await
        .expect("account");
        assert_eq!(
            account.address,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R"
        );
    }

    #[test]
    async fn ouis() {
        let client = Client::default();
        let ouis = accounts::ouis(
            &client,
            "13tyMLKRFYURNBQqLSqNJg9k41maP1A7Bh8QYxR13oWv7EnFooc",
        )
        .into_vec()
        .await
        .expect("oui list");
        assert_eq!(ouis.len(), 1);
    }

    #[test]
    async fn hotspots() {
        let client = Client::default();
        let hotspots = accounts::hotspots(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
        .into_vec()
        .await
        .expect("hotspot list");
        assert!(hotspots.len() > 0);
    }

    #[test]
    #[cfg(feature = "transactions")]
    async fn transactions() {
        let client = Client::default();
        let _txn = accounts::transactions(
            &client,
            "13WRNw4fmssJBvMqMnREwe1eCvUVXfnWXSXGcWXyVvAnQUF3D9R",
        )
            .next().await.expect("transactions").unwrap();
    }

    #[test]
    async fn richest() {
        let client = Client::default();
        let richest = accounts::richest(&client, Some(10))
            .await
            .expect("richest list");
        assert_eq!(richest.len(), 10);
    }
}
