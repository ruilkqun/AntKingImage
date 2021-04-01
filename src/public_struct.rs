use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageVersionJSONValue {
    pub image_version: HashMap<String,String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageLayerLayerDiffIDToLayerDigestJSONValue {
    pub image_digest_layerdiffid_to_layerdigest: HashMap<String,String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageLayerLayerDigestToLayerDiffIDJSONValue {
    pub image_digest_layerdigest_to_layerdiffid: HashMap<String,String>,
}