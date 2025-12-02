mod g2p;
mod handlers;
mod korean;
mod utils;

use std::{fs::File, io::BufReader, net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Context as _;
use axum::{routing::get, routing::post, Router};
use clap::Parser;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use vibrato::Dictionary;

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[derive(Clone)]
pub struct AppState {
    pub tokenizer: Arc<vibrato::Tokenizer>,
    pub detector: Arc<LanguageDetector>,
    pub maximum_length: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to listen on (e.g., 0.0.0.0:3000 or 127.0.0.1:3000)
    #[arg(env, long, default_value = "0.0.0.0:3000")]
    listen: SocketAddr,

    #[arg(
        env,
        long,
        default_value = "/usr/share/lingua/unidic-cwj-3_1_1/system.dic"
    )]
    dict: PathBuf,

    #[arg(env, long, default_value = "0")]
    maximum_length: usize,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load dictionary
    let file = File::open(&args.dict).context("Failed to open dict file")?;
    let dict = Dictionary::read(BufReader::new(file)).context("Failed to read as dictionary")?;
    let tokenizer = Arc::new(vibrato::Tokenizer::new(dict));

    // Initialize language detector
    let detector =
        Arc::new(LanguageDetectorBuilder::from_all_languages_without(&[Language::Chinese]).build());

    let state = AppState {
        tokenizer,
        detector,
        maximum_length: args.maximum_length,
    };

    // Configure routing
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/api/g2p", post(handlers::g2p))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind(&args.listen).await?;
    println!("Listening on {}", listener.local_addr()?);

    #[cfg(unix)]
    let mut sigterm = signal(SignalKind::terminate()).unwrap();

    #[cfg(unix)]
    tokio::select! {
        _ = tokio::signal::ctrl_c() => (),
        _ = sigterm.recv() => (),
        result = axum::serve(listener, app) => {
            result?;
        }
    }

    #[cfg(not(unix))]
    tokio::select! {
        _ = tokio::signal::ctrl_c() => (),
        result = axum::serve(listener, app) => {
            result?;
        }
    }

    Ok(())
}
