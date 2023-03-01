use mongodb::bson::{oid::ObjectId, to_document, Bson};
use serde::{Deserialize, Serialize};
use serde_json::Number;

#[derive(Serialize)]
pub struct Response<T, S> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<S>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Product {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description_html: String,
    pub price: Price,
}

impl From<Product> for Bson {
    fn from(product: Product) -> Bson {
        let doc = to_document(&product).unwrap();
        Bson::Document(doc)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InsertProduct {
    pub title: String,
    pub description_html: String,
    pub price: Price,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Price {
    pub value: Number,
    pub currency: Currency,
}
