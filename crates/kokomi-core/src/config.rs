use std::usize;
use crate::debug_fn_inline;
use crate::utils::ParseVar;
use colored::Colorize;

#[derive(Debug,Clone)]
pub struct KKMConfig {
    pub _use_gpu: bool,
    pub _model_path: String,
    pub _num_parallel: u8,
    pub _max_input_length: usize,
    pub _max_decoding_length: usize,
    pub _sampling_temperature: f32,
}

impl KKMConfig {
    pub fn init() -> KKMConfig {
        debug_fn_inline!();
        let _use_gpu = bool::parse_var("KKM_USE_GPU");
        let _model_path = String::parse_var_or("KKM_MODEL_PATH", "KKM_MODEL_PATH Not Specified!".to_string());
        let _num_parallel = usize::parse_var_or("KKM_N_PARALLEL", 1) as u8;
        let _max_input_length = usize::parse_var_or("KKM_INPUT_LEN", 2048);
        let _max_decoding_length = usize::parse_var_or("KKM_DECODE_LEN", 1920);
        let _sampling_temperature = f32::parse_var_or("KKM_TEMPERATURE", 0.1);
        KKMConfig {
            _use_gpu,
            _model_path,
            _num_parallel,
            _max_input_length,
            _max_decoding_length,
            _sampling_temperature,
        }
    }
}