
use mxyz_engine::config::EngineConfig;
use crate::simulation::renderer::config::RendererConfig;


pub struct Config {
    pub sim_id: String,
    pub engine_config: EngineConfig,
    pub renderer_config: RendererConfig,
    pub is_paused: bool,
}
impl Config {
    pub fn new(
        sim_id: &str,
    ) -> Self {
        Config {
            sim_id: String::from(sim_id),
            engine_config: EngineConfig::new(),
            renderer_config: RendererConfig::new(),
            is_paused: false,
        }
    }
}

