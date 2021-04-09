use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageVersionJSONValue {
    pub image_version: HashMap<String,HashMap<String,String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageDigestToNameVersionJSONValue {
    pub image_info: HashMap<String,Vec<String>>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageLayerLayerDiffIDToLayerDigestJSONValue {
    pub image_digest_layerdiffid_to_layerdigest: HashMap<String,String>,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageLayerLayerDigestToLayerDiffIDJSONValue {
    pub image_digest_layerdigest_to_layerdiffid: HashMap<String,String>,
}


// 写入数据库格式为image_id:{ layer_diff_id0:level0, layer_diff_id1:level1 }
// level为[0,1,2,3...]
#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageLayerLevelJSONValue {
    pub image_layer_level: HashMap<String,String>,
}



// 写入数据库格式为 image_chain_id:{ chainid0:{cacheid0,diffid0,parentchainid0,size0},chainid1:{cacheid1,diffid1,parentchainid1,size1} }
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ImageChainIDJSONValue {
    pub image_chain_id: HashMap<String,ImageChainIDItemJSONValue>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageChainIDItemJSONValue {
    pub cache_id: String,
    pub diff_id: String,
    pub parent_chain_id:String,
    pub size:String
}



#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageListItemJSONValue {
    pub id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests:Vec<String>,
    pub size:u64,
    pub uid:i64,
    pub username:String,
    pub spec: ImageSpecItemJSONValue
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ImageSpecItemJSONValue {
    pub image_digest: String,
    pub annotations: HashMap<String,String>
}