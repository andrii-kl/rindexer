use crate::rindexer::lens_registry_example::events::lens_registry::{
    no_extensions, HandleLinkedEvent, HandleUnlinkedEvent, LensRegistryEventType, NewEventOptions,
    NonceUpdatedEvent,
};
use rindexer_core::{
    generator::event_callback_registry::EventCallbackRegistry, EthereumSqlTypeWrapper,
};
use std::sync::Arc;

async fn handle_linked_handler(registry: &mut EventCallbackRegistry) {
    LensRegistryEventType::HandleLinked(
                    HandleLinkedEvent::new(
                        Arc::new(|results, context| {
                            Box::pin(async move {
                                println!("HandleLinked event: {:?}", results);
                                for result in results {
                            context
                                .database
                                .execute("INSERT INTO lens_registry_example.handle_linked (contract_address, \"handle_id\", \"handle_collection\", \"token_id\", \"token_collection\", \"transaction_executor\", \"timestamp\", \"tx_hash\", \"block_number\", \"block_hash\") VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                                &[&EthereumSqlTypeWrapper::Address(&result.tx_information.address),&EthereumSqlTypeWrapper::U256(&result.event_data.handle.id),&EthereumSqlTypeWrapper::Address(&result.event_data.handle.collection),&EthereumSqlTypeWrapper::U256(&result.event_data.token.id),&EthereumSqlTypeWrapper::Address(&result.event_data.token.collection),&EthereumSqlTypeWrapper::Address(&result.event_data.transaction_executor),&EthereumSqlTypeWrapper::U256(&result.event_data.timestamp),&EthereumSqlTypeWrapper::H256(&result.tx_information.transaction_hash.unwrap()),&EthereumSqlTypeWrapper::U64(&result.tx_information.block_number.unwrap()),&EthereumSqlTypeWrapper::H256(&result.tx_information.block_hash.unwrap())])
                                .await.unwrap();
                        }
        })
                        }),
                        no_extensions(),
                        NewEventOptions::default(),
                    )
                    .await,
                )
                .register(registry);
}

async fn handle_unlinked_handler(registry: &mut EventCallbackRegistry) {
    LensRegistryEventType::HandleUnlinked(
                    HandleUnlinkedEvent::new(
                        Arc::new(|results, context| {
                            Box::pin(async move {
                                println!("HandleUnlinked event: {:?}", results);
                                for result in results {
                    context
                        .database
                        .execute("INSERT INTO lens_registry_example.handle_unlinked (contract_address, \"handle_id\", \"handle_collection\", \"token_id\", \"token_collection\", \"transaction_executor\", \"timestamp\", \"tx_hash\", \"block_number\", \"block_hash\") VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
                        &[&EthereumSqlTypeWrapper::Address(&result.tx_information.address),&EthereumSqlTypeWrapper::U256(&result.event_data.handle.id),&EthereumSqlTypeWrapper::Address(&result.event_data.handle.collection),&EthereumSqlTypeWrapper::U256(&result.event_data.token.id),&EthereumSqlTypeWrapper::Address(&result.event_data.token.collection),&EthereumSqlTypeWrapper::Address(&result.event_data.transaction_executor),&EthereumSqlTypeWrapper::U256(&result.event_data.timestamp),&EthereumSqlTypeWrapper::H256(&result.tx_information.transaction_hash.unwrap()),&EthereumSqlTypeWrapper::U64(&result.tx_information.block_number.unwrap()),&EthereumSqlTypeWrapper::H256(&result.tx_information.block_hash.unwrap())])
                        .await.unwrap();
                }
        })
                        }),
                        no_extensions(),
                        NewEventOptions::default(),
                    )
                    .await,
                )
                .register(registry);
}

async fn nonce_updated_handler(registry: &mut EventCallbackRegistry) {
    LensRegistryEventType::NonceUpdated(
                    NonceUpdatedEvent::new(
                        Arc::new(|results, context| {
                            Box::pin(async move {
                                println!("NonceUpdated event: {:?}", results);
                                for result in results {
                    context
                        .database
                        .execute("INSERT INTO lens_registry_example.nonce_updated (contract_address, \"signer\", \"nonce\", \"timestamp\", \"tx_hash\", \"block_number\", \"block_hash\") VALUES($1, $2, $3, $4, $5, $6, $7)",
                        &[&EthereumSqlTypeWrapper::Address(&result.tx_information.address),&EthereumSqlTypeWrapper::Address(&result.event_data.signer),&EthereumSqlTypeWrapper::U256(&result.event_data.nonce),&EthereumSqlTypeWrapper::U256(&result.event_data.timestamp),&EthereumSqlTypeWrapper::H256(&result.tx_information.transaction_hash.unwrap()),&EthereumSqlTypeWrapper::U64(&result.tx_information.block_number.unwrap()),&EthereumSqlTypeWrapper::H256(&result.tx_information.block_hash.unwrap())])
                        .await.unwrap();
                }
        })
                        }),
                        no_extensions(),
                        NewEventOptions::default(),
                    )
                    .await,
                )
                .register(registry);
}
pub async fn lens_registry_handlers(registry: &mut EventCallbackRegistry) {
    handle_linked_handler(registry).await;

    handle_unlinked_handler(registry).await;

    nonce_updated_handler(registry).await;
}