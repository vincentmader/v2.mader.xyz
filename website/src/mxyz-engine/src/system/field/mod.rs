
pub struct Field {
    pub state_vector:   Vec<FieldAttribute>,
    pub config:         crate::config::field::FieldEngineConfig,
}
impl Field {
    pub fn new() -> Self {
        let id = 0; // TODO remove from function args
        Field {
            state_vector:   Vec::new(),
            config:         crate::config::field::FieldEngineConfig::new(id),
        }
    }
}

pub enum FieldAttribute {
    Density(Vec<f64>),
    Spin(Vec<bool>),
    VelocityX(Vec<f64>),
    VelocityY(Vec<f64>),
    VelocityZ(Vec<f64>),
}

