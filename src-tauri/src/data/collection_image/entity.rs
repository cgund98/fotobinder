#[derive(serde::Serialize, Clone)]
pub struct CollectionImage {
    pub relative_path: String,
    pub source_id: String,
    pub collection_id: String,
}

pub type DbCollectionImage = CollectionImage;
