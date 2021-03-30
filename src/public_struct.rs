use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageVersionJSONValue {
    pub image_version: HashMap<String,String>,
}