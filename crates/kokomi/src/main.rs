use std::sync::Arc;
use akira::{LlamaGeneration, LlamaGenerationOptionsBuilder};
use kokomi_core::config::KKMConfig;
use kokomi_core::debug_var;
use colored::Colorize;
use axum::Router;
use axum::routing::{get, post};
use crate::router::completion::v1_completion;
use crate::router::model::v1_model;
use crate::state::KKMState;

mod router;
mod state;

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
        .route("/model", get(v1_model))
        .route("/completion", post(v1_completion))
        .with_state(shared_config);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:11451").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}