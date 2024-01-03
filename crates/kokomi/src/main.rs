use akira::{LlamaGeneration, LlamaGenerationOptionsBuilder, TextGeneration, TextGenerationOptionsBuilder};
use kokomi_core::config::KKMConfig;

#[tokio::main]
async fn main() {
    let config = KKMConfig::init();

    let option = LlamaGenerationOptionsBuilder::default()
        .use_gpu(config._use_gpu)
        .model_path(config._model_path)
        .parallelism(config._num_parallel)
        .build().unwrap();
    let llama = LlamaGeneration::new(option);

    let gen_option = TextGenerationOptionsBuilder::default()
        .max_input_length(config._max_input_length)
        .max_decoding_length(config._max_decoding_length)
        .sampling_temperature(config._sampling_temperature)
        .build().unwrap();
    let res = llama.generate("#include<iostream>\nusing namespace std;\n\n// a gcd function", gen_option).await;
    println!("{}", res);
}