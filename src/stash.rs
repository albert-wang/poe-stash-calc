use config;

use serde::{Deserialize};
use serde_json;


#[derive(Deserialize)]
pub struct Item {
    #[serde(rename= "typeLine")]
    type_lnie: String,

    #[serde(rename = "stackSize")]
    count: i32,
}

#[derive(Deserialize)]
pub struct CurrencyTab {
    #[serde(rename(deserialize = "numTabs"))]
    pub num_tabs: i32 ,

    pub items: Vec<Item>,
}

pub fn load_currency_stash(cfg: &config::Configuration, index: i32) -> Result<CurrencyTab, String> {
    return Err("Not implemented".to_string())
}