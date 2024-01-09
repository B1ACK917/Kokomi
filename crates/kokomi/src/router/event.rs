use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};

use kokomi_core::debug_fn;
use colored::Colorize;
use crate::state::KKMState;

pub async fn v1_event(State(state): State<Box<Arc<KKMState>>>) -> Json<Value> {
    debug_fn!();
    return Json(json!({
        "model":state.config._model_path
    }));
}