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
    pub _id: String,
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
