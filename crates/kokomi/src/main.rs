use std::sync::Arc;
use akira::{LlamaGeneration, LlamaGenerationOptionsBuilder};
use kokomi_core::config::KKMConfig;
use kokomi_core::debug_var;
use colored::Colorize;
use axum::Router;
use axum::routing::{get, post};
use crate::router::completion::v1_completion;
use crate::router::event::v1_event;
use crate::router::health::v1_health;
use crate::state::KKMState;

mod router;
mod state;
mod consts;

#[tokio::main]
async fn main() {
    let build_config = KKMConfig::init();
    debug_var!(build_config);
    let config = build_config.clone();

    let option = LlamaGenerationOptionsBuilder::default()
        .use_gpu(build_config._use_gpu)
        .model_path(build_config._model_path)
        .parallelism(build_config._num_parallel)
        .build().unwrap();
    let llama = LlamaGeneration::new(option);
    let shared_config = Box::new(Arc::new(KKMState::from(config, llama)));

    let app = Router::new()
        .route("/v1/health", get(v1_health))
        .route("/v1/events", post(v1_event))
        .route("/v1/completions", post(v1_completion))
        .with_state(shared_config);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11451").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}