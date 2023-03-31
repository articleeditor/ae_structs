use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Collection {
    pub _id: Option<ObjectId>,
    pub product_ids: Option<Vec<String>>,
    pub shop_ids: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateCollection {
    pub product_ids: Option<Vec<String>>,
    pub shop_ids: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
