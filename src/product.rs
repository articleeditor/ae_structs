use mongodb::bson::{oid::ObjectId, to_document, Bson};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Product {
    pub _id: Option<ObjectId>,
    pub shopify_id: Option<String>,
    pub title: Option<String>,
    pub description_html: Option<String>,
    pub price: Option<Price>,
}

impl From<Product> for Bson {
    fn from(product: Product) -> Bson {
        let doc = to_document(&product).unwrap();
        Bson::Document(doc)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MergeProduct {
    pub shopify_id: Option<String>,
    pub title: Option<String>,
    pub description_html: Option<String>,
    pub price: Option<Price>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Price {
    pub value: Number,
    pub currency: Currency,
}
