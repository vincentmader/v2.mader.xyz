
use mxyz_engine::config::field::FieldEngineConfig;


pub struct FieldRendererConfig {

    pub id: usize,
    pub is_displayed: bool,

}
impl FieldRendererConfig {

    pub fn new(
        _sim_id: &str,  // TODO move initialization somewhere else?
        id: usize, 
        _field: &FieldEngineConfig
    ) -> Self {
        FieldRendererConfig {
            id: id,
            is_displayed: true,
        }
    }
}

