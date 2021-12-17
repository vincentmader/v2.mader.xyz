
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    id: String,
    title: String,
    path_to_thumbnail: String,
    // subsections
}
impl Section {
    pub fn new(
        id: &str, 
        title: &str,
    ) -> Section {
        let mut path_to_thumbnail = String::from("/static/img/thumbnails/");
        path_to_thumbnail.push_str(&id);
        path_to_thumbnail.push_str(".png");
        return Section {
            id: String::from(id),
            title: String::from(title),
            path_to_thumbnail: path_to_thumbnail, 
        };
    }
}

