
pub struct ObjFamily {
    pub state_vector:   Vec<ObjAttribute>,
    pub config:         crate::config::obj_family::ObjFamilyEngineConfig,
}
impl ObjFamily {
    pub fn new() -> Self {
        let id = 0; // TODO remove
        ObjFamily {
            state_vector:   Vec::new(),
            config:         crate::config::obj_family::ObjFamilyEngineConfig::new(id),
        }
    }
}

pub enum ObjAttribute {
    Charge(Vec<f64>),
    Mass(Vec<f64>),
    PositionX(Vec<f64>),
    PositionY(Vec<f64>),
    PositionZ(Vec<f64>),
    Spin(Vec<bool>),
    VelocityX(Vec<f64>),
    VelocityY(Vec<f64>),
    VelocityZ(Vec<f64>),
}
