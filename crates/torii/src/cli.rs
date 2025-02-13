use clap::Parser;
use graphql::server::start_graphql;
use num::{BigUint, Num};
use sqlx::sqlite::SqlitePoolOptions;
use starknet::providers::jsonrpc::HttpTransport;
use starknet::providers::JsonRpcClient;
use storage::sql::SqlStorage;
use tokio_util::sync::CancellationToken;
use tracing::error;
use tracing_subscriber::fmt;
use url::Url;

use crate::indexer::start_indexer;

mod processors;

mod graphql;
mod indexer;
mod storage;
mod tests;

/// Dojo World Indexer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The world to index
    #[arg(short, long, default_value = "0x420")]
    world: String,
    /// The rpc endpoint to use
    #[arg(long, default_value = "http://localhost:5050")]
    rpc: String,
    /// Database url
    #[arg(short, long, default_value = "sqlite::memory:")]
    database_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let subscriber = fmt::Subscriber::builder()
        .with_max_level(tracing::Level::INFO) // Set the maximum log level
        .finish();

    // Set the global subscriber
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set the global tracing subscriber");

    // Setup cancellation for graceful shutdown
    let cts = CancellationToken::new();
    ctrlc::set_handler({
        let cts: CancellationToken = cts.clone();
        move || {
            cts.cancel();
        }
    })?;

    let world = BigUint::from_str_radix(&args.world[2..], 16).unwrap_or_else(|error| {
        panic!("Failed parsing world address: {error:?}");
    });

    let database_url = &args.database_url;
    #[cfg(feature = "sqlite")]
    let pool = SqlitePoolOptions::new().max_connections(5).connect(database_url).await?;
    let provider = JsonRpcClient::new(HttpTransport::new(Url::parse(&args.rpc).unwrap()));

    let storage = SqlStorage::new(pool.clone())?;
    let indexer = start_indexer(cts.clone(), world, &storage, &provider);

    let graphql = start_graphql(&pool);

    tokio::select! {
        res = indexer => {
            if let Err(e) = res {
                error!("Indexer failed with error: {:?}", e);
            }
        }
        res = graphql => {
            if let Err(e) = res {
                error!("GraphQL server failed with error: {:?}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down");
        }
    }

    Ok(())
}
