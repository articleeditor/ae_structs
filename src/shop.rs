use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Shop {
    pub _id: Option<ObjectId>,
    pub user_ids: Option<Vec<String>>,
    pub name: Option<String>,
    pub platform: Option<Platform>,
    pub api_key: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MergeShop {
    pub user_ids: Option<Vec<String>>,
    pub name: Option<String>,
    pub platform: Option<Platform>,
    pub api_key: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Platform {
    Shopify,
    WooCommerce,
}
