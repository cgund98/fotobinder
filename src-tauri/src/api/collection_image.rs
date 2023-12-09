use crate::data::collection_image::entity;

#[derive(serde::Serialize)]
pub struct CollectionImages {
    pub collection_images: Vec<entity::CollectionImage>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TagAssignments {
    pub add: Vec<String>,
    pub remove: Vec<String>,
}

impl From<Vec<entity::CollectionImage>> for CollectionImages {
    fn from(e: Vec<entity::CollectionImage>) -> Self {
        Self {
            collection_images: e,
        }
    }
}
