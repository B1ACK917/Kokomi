use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use kokomi_core::debug_fn;
use colored::Colorize;
use akira::{TextGeneration, TextGenerationOptionsBuilder};
use serde::Deserialize;
use crate::state::KKMState;

#[derive(Debug,Deserialize)]
pub struct CompletionBody {
    pub(crate) input: String,
}

pub async fn v1_completion(State(state): State<Box<Arc<KKMState>>>,
                           Json(body): Json<CompletionBody>) -> Json<Value> {
    debug_fn!(body);
    let gen_option = TextGenerationOptionsBuilder::default()
        .max_input_length(state.config._max_input_length)
        .max_decoding_length(state.config._max_decoding_length)
        .sampling_temperature(state.config._sampling_temperature)
        .build().unwrap();
    let res = state.model.generate(&body.input, gen_option).await;
    return Json(json!({
        "input":body.input,
        "response":res
    }));
}