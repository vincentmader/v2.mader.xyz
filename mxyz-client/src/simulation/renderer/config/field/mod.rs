

pub struct FieldRendererConfig {

    pub id: usize,
    pub is_displayed: bool,

}
impl FieldRendererConfig {

    pub fn new(id: usize) -> Self {
        FieldRendererConfig {
            id: id,
            is_displayed: true,
        }
    }
}

