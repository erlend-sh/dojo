use beerus_core::lightclient::beerus::BeerusLightClient;
use bevy::app::{App, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{IntoPipeSystem, Query};
use bevy_tokio_tasks::TaskContext;
use eyre::Result;

use crate::light_client::{handle_errors, BlockNumber, LightClient, LightClientRequest};

pub struct StarknetClientPlugin;

impl Plugin for StarknetClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StarknetGetBlockWithTxHashes>()
            .add_event::<StarknetGetBlockWithTxs>()
            .add_event::<StarknetGetStateUpdate>()
            .add_event::<StarknetGetStorageAt>()
            .add_event::<StarknetGetTransactionByHash>()
            .add_event::<StarknetGetTransactionByBlockIdAndIndex>()
            .add_event::<StarknetGetTransactionReceipt>()
            .add_event::<StarknetGetClass>()
            .add_event::<StarknetGetClassHashAt>()
            .add_event::<StarknetGetClassAt>()
            .add_event::<StarknetGetBlockTransactionCount>()
            .add_event::<StarknetCall>()
            .add_event::<StarknetEstimateFee>()
            .add_event::<StarknetBlockNumber>()
            .add_event::<StarknetBlockHashAndNumber>()
            .add_event::<StarknetChainId>()
            .add_event::<StarknetPendingTransactions>()
            .add_event::<StarknetSyncing>()
            .add_event::<StarknetGetEvents>()
            .add_event::<StarknetGetNonce>()
            .add_event::<StarknetL1ToL2Messages>()
            .add_event::<StarknetL1ToL2MessageNonce>()
            .add_event::<StarknetL1ToL2MessageCancellations>()
            .add_event::<StarknetL2ToL1Messages>()
            .add_event::<StarknetAddDeclareTransaction>()
            .add_event::<StarknetAddDeployAccountTransaction>()
            .add_event::<StarknetGetContractStorageProof>()
            .add_event::<StarknetAddInvokeTransaction>()
            .add_systems((
                get_block_with_tx_hashes.pipe(handle_errors),
                block_number.pipe(handle_errors),
            ));
    }
}

////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////

// TODO: parameters
pub struct StarknetGetBlockWithTxHashes;

pub struct StarknetGetBlockWithTxs;

pub struct StarknetGetStateUpdate;

pub struct StarknetGetStorageAt;

pub struct StarknetGetTransactionByHash;

pub struct StarknetGetTransactionByBlockIdAndIndex;

pub struct StarknetGetTransactionReceipt;

pub struct StarknetGetClass;

pub struct StarknetGetClassHashAt;

pub struct StarknetGetClassAt;

pub struct StarknetGetBlockTransactionCount;

pub struct StarknetCall;

pub struct StarknetEstimateFee;

pub struct StarknetBlockNumber;

pub struct StarknetBlockHashAndNumber;

pub struct StarknetChainId;

pub struct StarknetPendingTransactions;

pub struct StarknetSyncing;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct StarknetGetEvents;

pub struct StarknetGetNonce;

pub struct StarknetL1ToL2Messages;

pub struct StarknetL1ToL2MessageNonce;

pub struct StarknetL1ToL2MessageCancellations;

pub struct StarknetL2ToL1Messages;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct StarknetAddDeclareTransaction;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct StarknetAddDeployAccountTransaction;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct StarknetGetContractStorageProof;

/// Not supported: https://github.com/keep-starknet-strange/beerus#endpoint-support
pub struct StarknetAddInvokeTransaction;

////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////

/// Handle [StarknetGetBlockWithTxHashes] request event
///
/// ### TODO
/// Provide proc macro to reduce boilerplate?
fn get_block_with_tx_hashes(
    mut events: EventReader<StarknetGetBlockWithTxHashes>,
    query: Query<&LightClient>,
) -> Result<()> {
    events.iter().try_for_each(|_e| {
        let node = query.get_single()?;
        node.request(LightClientRequest::starknet_get_block_with_tx_hashes())?;

        Ok(())
    })
}

/// Handle [StarknetBlockNumber] request event
fn block_number(
    mut events: EventReader<StarknetBlockNumber>,
    query: Query<&LightClient>,
) -> Result<()> {
    events.iter().try_for_each(|_e| {
        let node = query.get_single()?;
        node.request(LightClientRequest::starknet_block_number())?;

        Ok(())
    })
}

////////////////////////////////////////////////////////////////////////
// Utils
////////////////////////////////////////////////////////////////////////

use StarknetRequest::*;
impl LightClientRequest {
    pub fn starknet_get_block_with_tx_hashes() -> Self {
        Self::Starknet(GetBlockWithTxHashes)
    }

    pub fn starknet_block_number() -> Self {
        Self::Starknet(BlockNumber)
    }
}

#[derive(Debug)]
pub enum StarknetRequest {
    GetBlockWithTxHashes,
    BlockNumber,
}

impl StarknetRequest {
    pub async fn get_block_with_tx_hashes(
        _client: &BeerusLightClient,
        mut _ctx: TaskContext,
    ) -> Result<()> {
        todo!();
    }

    pub async fn block_number(client: &BeerusLightClient, mut ctx: TaskContext) -> Result<()> {
        let block_number = client.starknet_lightclient.block_number().await?;

        ctx.run_on_main_thread(move |ctx| {
            ctx.world.spawn((Starknet, BlockNumber::new(block_number)));
        })
        .await;

        Ok(())
    }
}

/// Labeling component for Starknet related entity
#[derive(Component)]
pub struct Starknet;
