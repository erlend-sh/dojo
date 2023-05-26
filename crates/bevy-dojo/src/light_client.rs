//! Beerus based Ethereum/Starknet light client module.
//!
//! More on [Beerus GitHub page](https://github.com/keep-starknet-strange/beerus)
//!
//! ### Usage
//!
//! ```rust, no_run
//! use bevy::prelude::*;
//! use bevy_dojo::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         // Set up Starknet light client plugin.
//!         .add_plugin(LightClientPlugin)
//!         .run();
//! }
//! ```

pub mod ethereum;
pub mod starknet;

pub mod prelude {
    pub use ethereum::*;
    pub use starknet::*;

    pub use crate::light_client::*;
}

use std::process::exit;

use beerus_core::config::Config;
use beerus_core::lightclient::beerus::BeerusLightClient;
use beerus_core::lightclient::ethereum::helios_lightclient::HeliosLightClient;
use beerus_core::lightclient::starknet::StarkNetLightClientImpl;
use bevy::app::Plugin;
use bevy::ecs::component::Component;
use bevy::ecs::system::{In, ResMut};
use bevy::log;
use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};
use eyre::Result;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::mpsc::Sender;

use self::ethereum::{EthRequest, EthereumClientPlugin};
use self::starknet::{StarknetClientPlugin, StarknetRequest};

/// Plugin to manage Ethereum/Starknet light client.
pub struct LightClientPlugin;

impl Plugin for LightClientPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(TokioTasksPlugin::default())
            .add_plugin(EthereumClientPlugin)
            .add_plugin(StarknetClientPlugin)
            .add_startup_system(start_beerus);
    }
}

fn start_beerus(runtime: ResMut<'_, TokioTasksRuntime>) {
    log::info!("Starting...");
    let config = Config::from_env();
    let (tx, mut rx) = tokio::sync::mpsc::channel::<LightClientRequest>(1);

    runtime.spawn_background_task(|mut ctx| async move {
        log::info!("Creating ethereum(helios) lightclient...");
        let ethereum_lightclient = match HeliosLightClient::new(config.clone()).await {
            Ok(ethereum_lightclient) => ethereum_lightclient,
            Err(err) => {
                log::error! {"{}", err};
                exit(1);
            }
        };

        log::info!("Creating starknet lightclient...");
        let starknet_lightclient = match StarkNetLightClientImpl::new(&config) {
            Ok(starknet_lightclient) => starknet_lightclient,
            Err(err) => {
                log::error! {"{}", err};
                exit(1);
            }
        };

        log::info!("Creating beerus lightclient...");
        let mut client = BeerusLightClient::new(
            config,
            Box::new(ethereum_lightclient),
            Box::new(starknet_lightclient),
        );

        match client.start().await {
            Ok(_) => {
                log::info!("Light client is ready");

                ctx.run_on_main_thread(move |ctx| {
                    ctx.world.spawn(LightClient::new(tx));
                })
                .await;

                while let Some(req) = rx.recv().await {
                    log::info!("Node request: {:?}", req);

                    let ctx = ctx.clone();
                    let res = match req {
                        LightClientRequest::Starknet(starknet_req) => {
                            use StarknetRequest::*;

                            match starknet_req {
                                GetBlockWithTxHashes => {
                                    StarknetRequest::get_block_with_tx_hashes(&client, ctx).await
                                }
                                BlockNumber => StarknetRequest::block_number(&client, ctx).await,
                            }
                        }
                        LightClientRequest::Ethereum(ethereum_req) => {
                            use EthRequest::*;

                            match ethereum_req {
                                GetBlockNumber => EthRequest::get_block_number(&client, ctx).await,
                            }
                        }
                    };

                    if let Err(e) = res {
                        log::error!("{e}");
                    }
                }
            }
            Err(e) => {
                log::error!("{e}");
            }
        }
    });
}

#[derive(Component)]
pub struct LightClient {
    tx: Sender<LightClientRequest>,
}

impl LightClient {
    fn new(tx: Sender<LightClientRequest>) -> Self {
        Self { tx }
    }

    pub fn request(&self, req: LightClientRequest) -> Result<(), TrySendError<LightClientRequest>> {
        self.tx.try_send(req)
    }
}

// TODO: Support all methods
// TODO: Should we expose it as a component bundle instead? Then, add systems to convert it as enum.
#[derive(Debug)]
pub enum LightClientRequest {
    Ethereum(EthRequest),
    Starknet(StarknetRequest),
}

#[derive(Component)]
pub struct NodeResponse;

#[derive(Component)]
pub struct BlockNumber {
    pub value: u64,
}

impl BlockNumber {
    fn new(value: u64) -> Self {
        Self { value }
    }
}

fn handle_errors(In(result): In<Result<()>>) {
    if let Err(e) = result {
        log::error!("{e}");
    }
}
