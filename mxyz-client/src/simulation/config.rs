
// use mxyz_engine::config::EngineConfig;
// use crate::simulation::renderer::config::RendererConfig;


pub struct Config {
    pub sim_id: String,
    // pub engine: EngineConfig,
    // pub renderer: RendererConfig,
    pub is_paused: bool,
    pub nr_of_steps_per_render: usize,
    // TODO time_step: 
}
impl Config {
    pub fn new(
        sim_id: &str,
    ) -> Self {
        Config {
            sim_id: String::from(sim_id),
            // engine: EngineConfig::new(),
            // renderer: RendererConfig::new(),
            is_paused: true, // TODO unused atm
            nr_of_steps_per_render: 1,
        }
    }
}

