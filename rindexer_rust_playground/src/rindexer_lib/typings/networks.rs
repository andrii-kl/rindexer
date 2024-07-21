use std::sync::Arc;

/// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
///
/// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
/// Any manual changes to this file will be overwritten.
use ethers::providers::{Http, Provider, RetryClient};
use ethers::types::U64;
use rindexer::{
    lazy_static,
    provider::{create_client, JsonRpcCachedProvider},
    public_read_env_value,
};

lazy_static! {
    static ref ETHEREUM_PROVIDER: Arc<JsonRpcCachedProvider> = create_client(
        &public_read_env_value("https://mainnet.gateway.tenderly.co")
            .unwrap_or("https://mainnet.gateway.tenderly.co".to_string()),
        None,
        None
    )
    .expect("Error creating provider");
    static ref YOMINET_PROVIDER: Arc<JsonRpcCachedProvider> = create_client(
        &public_read_env_value("https://yominet.rpc.caldera.xyz/http")
            .unwrap_or("https://yominet.rpc.caldera.xyz/http".to_string()),
        None,
        Some(U64::from(10000))
    )
    .expect("Error creating provider");
}
pub fn get_ethereum_provider_cache() -> Arc<JsonRpcCachedProvider> {
    Arc::clone(&ETHEREUM_PROVIDER)
}

pub fn get_ethereum_provider() -> Arc<Provider<RetryClient<Http>>> {
    ETHEREUM_PROVIDER.get_inner_provider()
}

pub fn get_yominet_provider_cache() -> Arc<JsonRpcCachedProvider> {
    Arc::clone(&YOMINET_PROVIDER)
}

pub fn get_yominet_provider() -> Arc<Provider<RetryClient<Http>>> {
    YOMINET_PROVIDER.get_inner_provider()
}

pub fn get_provider_cache_for_network(network: &str) -> Arc<JsonRpcCachedProvider> {
    if network == "ethereum" {
        return get_ethereum_provider_cache();
    }

    if network == "yominet" {
        return get_yominet_provider_cache();
    }
    panic!("Network not supported")
}
