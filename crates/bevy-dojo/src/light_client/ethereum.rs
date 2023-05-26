use beerus_core::lightclient::beerus::BeerusLightClient;
use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{IntoPipeSystem, Query};
use bevy_tokio_tasks::TaskContext;
use eyre::Result;

use super::handle_errors;
use crate::light_client::{BlockNumber, LightClient, LightClientRequest};

pub struct EthereumClientPlugin;

impl Plugin for EthereumClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EthGetBalance>()
            .add_event::<EthGetTransactionCount>()
            .add_event::<EthGetCode>()
            .add_event::<EthCall>()
            .add_event::<EthEstimateGas>()
            .add_event::<EthGetChainId>()
            .add_event::<EthGasPrice>()
            .add_event::<EthMaxPriorityFeePerGas>()
            .add_event::<EthBlockNumber>()
            .add_event::<EthGetBlockByNumber>()
            .add_event::<EthGetBlockByHash>()
            .add_event::<EthSendRawTransaction>()
            .add_event::<EthGetTransactionReceipt>()
            .add_event::<EthGetLogs>()
            .add_event::<EthGetStorageAt>()
            .add_event::<EthGetBlockTransactionCountByHash>()
            .add_event::<EthGetBlockTransactionCountByNumber>()
            .add_event::<EthCoinbase>()
            .add_event::<EthSyncing>()
            .add_event::<EthGetTransactionByHash>()
            .add_event::<EthGetBlockNumber>()
            .add_system(get_block_number.pipe(handle_errors));
    }
}

////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////

pub struct EthGetBalance;

pub struct EthGetTransactionCount;

pub struct EthGetCode;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct EthCall;

pub struct EthEstimateGas;

pub struct EthGetChainId;

pub struct EthGasPrice;

pub struct EthMaxPriorityFeePerGas;

pub struct EthBlockNumber;

pub struct EthGetBlockByNumber;

pub struct EthGetBlockByHash;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct EthSendRawTransaction;

pub struct EthGetTransactionReceipt;

pub struct EthGetLogs;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct EthGetStorageAt;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct EthGetBlockTransactionCountByHash;

pub struct EthGetBlockTransactionCountByNumber;

pub struct EthCoinbase;

pub struct EthSyncing;

pub struct EthGetTransactionByHash;

pub struct EthGetTransactionByBlockHashAndIndex;

pub struct EthGetBlockNumber;

////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////

/// React to [EthGetBlockNumber] event
fn get_block_number(
    mut events: EventReader<EthGetBlockNumber>,
    query: Query<&LightClient>,
) -> Result<()> {
    events.iter().try_for_each(|_e| {
        let node = query.get_single()?;
        node.request(LightClientRequest::ethereum_get_block_number())?;

        Ok(())
    })
}

////////////////////////////////////////////////////////////////////////
// Utils
////////////////////////////////////////////////////////////////////////

use EthRequest::*;
impl LightClientRequest {
    pub fn ethereum_get_block_number() -> Self {
        Self::Ethereum(GetBlockNumber)
    }
}

#[derive(Debug)]
pub enum EthRequest {
    GetBlockNumber,
}

impl EthRequest {
    pub async fn get_block_number(
        client: &BeerusLightClient,
        mut ctx: TaskContext,
    ) -> eyre::Result<()> {
        let block_number = client.ethereum_lightclient.lock().await.get_block_number().await?;

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.spawn((Ethereum, BlockNumber::new(block_number)));
        })
        .await;

        Ok(())
    }
}

/// Labeling component for Ethereum related entity
#[derive(Component)]
pub struct Ethereum;
