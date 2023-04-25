
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct NavGridItem {
    item_id: String,
    title: String,
    path_to_thumbnail: String,
}
impl NavGridItem {
    pub fn new(item_id: &str, title: &str) -> Self {

        let path_to_thumbnail = format!(
            "/static/img/thumbnails/{}.png", item_id
        );

        NavGridItem {
            item_id: String::from(item_id),
            title: String::from(title),
            path_to_thumbnail,
        }
    }
}

