use akira::LlamaGeneration;
use kokomi_core::config::KKMConfig;

pub struct KKMState {
    pub(crate) config: KKMConfig,
    pub(crate) model: LlamaGeneration,
}

impl KKMState {
    pub fn from(config: KKMConfig, model: LlamaGeneration) -> Self {
        return Self {
            config,
            model,
        };
    }
}