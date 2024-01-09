use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use kokomi_core::debug_fn;
use colored::Colorize;
use akira::{TextGeneration, TextGenerationOptionsBuilder};
use serde::{Deserialize, Serialize};
use kokomi_core::utils::gen_rand_str;
use crate::consts::KKM_DEFAULT_UUID_LEN;
use crate::state::KKMState;

#[derive(Debug, Deserialize, Serialize)]
pub struct Segment {
    pub(crate) prefix: String,
    pub(crate) suffix: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompletionBody {
    pub(crate) segments: Segment,
}

pub async fn v1_completion(State(state): State<Box<Arc<KKMState>>>,
                           Json(body): Json<CompletionBody>) -> Json<Value> {
    debug_fn!(body);
    let gen_option = TextGenerationOptionsBuilder::default()
        .max_input_length(state.config._max_input_length)
        .max_decoding_length(state.config._max_decoding_length)
        .sampling_temperature(state.config._sampling_temperature)
        .build().unwrap();
    let res = state.model.generate(&body.segments.prefix, gen_option).await;
    let id = "UUID-".to_string() + (&gen_rand_str(KKM_DEFAULT_UUID_LEN).to_uppercase());
    return Json(json!({
        "id":id,
        "choices":[
            {
                "index":0,
                "text":res
            }
        ],
        "input":body.segments
    }));
}