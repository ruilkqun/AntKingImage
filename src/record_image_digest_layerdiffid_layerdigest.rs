use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageLayerLayerDiffIDToLayerDigestJSONValue,ImageLayerLayerDigestToLayerDiffIDJSONValue };

pub async fn record_image_digest_layer_diff_id_to_layer_digest(db: &sled::Db,image_digest:String,layer_diff_id:String,layer_digest:String) -> sled::Result<()> {
    let tree = TreeWrapper::<JSONEncoder<ImageLayerLayerDiffIDToLayerDigestJSONValue>, JSONEncoder<ImageLayerLayerDiffIDToLayerDigestJSONValue>>::new(
        db.open_tree("image_digest_layerdiffid_to_layerdigest")?,
    );

    let value = tree
    .get(image_digest.clone());

    let image_digest_json_value;
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            res2.image_digest_layerdiffid_to_layerdigest.insert(layer_diff_id.clone(),layer_digest.clone());
                            image_digest_json_value = ImageLayerLayerDiffIDToLayerDigestJSONValue {
                                image_digest_layerdiffid_to_layerdigest: res2.image_digest_layerdiffid_to_layerdigest
                            };
                        }
                        _ => {
                            let mut image_digest_layerdiffid_to_layerdigest = HashMap::new();
                            image_digest_layerdiffid_to_layerdigest.insert(layer_diff_id.clone(),layer_digest.clone());
                            image_digest_json_value = ImageLayerLayerDiffIDToLayerDigestJSONValue {
                                image_digest_layerdiffid_to_layerdigest
                            };
                        }
                    }
                },
                _ => {
                        let mut image_digest_layerdiffid_to_layerdigest = HashMap::new();
                        image_digest_layerdiffid_to_layerdigest.insert(layer_diff_id.clone(),layer_digest.clone());
                        image_digest_json_value = ImageLayerLayerDiffIDToLayerDigestJSONValue {
                            image_digest_layerdiffid_to_layerdigest
                        };
                }
            }
        },
        Err(_) => {
            let mut image_digest_layerdiffid_to_layerdigest = HashMap::new();
            image_digest_layerdiffid_to_layerdigest.insert(layer_diff_id.clone(),layer_digest.clone());
            image_digest_json_value = ImageLayerLayerDiffIDToLayerDigestJSONValue {
                image_digest_layerdiffid_to_layerdigest
            };
        }
    };

    tree.insert(image_digest.clone(), &image_digest_json_value)?;
    // let value1 = tree
    //     .get(image_digest.clone())?
    //     .expect("Value not found")
    //     .decode()
    //     .expect("Decoding failed");
    // assert_eq!(value, image_repositories_json_value);
    // println!("image_digest:{:?}",value1);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}



pub async fn record_image_digest_layer_digest_layer_diff_id(db: &sled::Db,image_digest:String,layer_diff_id:String,layer_digest:String) -> sled::Result<()> {
    let tree = TreeWrapper::<JSONEncoder<ImageLayerLayerDigestToLayerDiffIDJSONValue>, JSONEncoder<ImageLayerLayerDigestToLayerDiffIDJSONValue>>::new(
        db.open_tree("image_digest_layerdigest_to_layerdiffid_to")?,
    );

    let value = tree
    .get(image_digest.clone());

    let image_digest_json_value;
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            res2.image_digest_layerdigest_to_layerdiffid.insert(layer_diff_id.clone(),layer_digest.clone());
                            image_digest_json_value = ImageLayerLayerDigestToLayerDiffIDJSONValue {
                                image_digest_layerdigest_to_layerdiffid: res2.image_digest_layerdigest_to_layerdiffid
                            };
                        }
                        _ => {
                            let mut image_digest_layerdigest_to_layerdiffid = HashMap::new();
                            image_digest_layerdigest_to_layerdiffid.insert(layer_diff_id.clone(),layer_digest.clone());
                            image_digest_json_value = ImageLayerLayerDigestToLayerDiffIDJSONValue {
                                image_digest_layerdigest_to_layerdiffid
                            };
                        }
                    }
                },
                _ => {
                        let mut image_digest_layerdigest_to_layerdiffid = HashMap::new();
                        image_digest_layerdigest_to_layerdiffid.insert(layer_diff_id.clone(),layer_digest.clone());
                        image_digest_json_value = ImageLayerLayerDigestToLayerDiffIDJSONValue {
                            image_digest_layerdigest_to_layerdiffid
                        };
                }
            }
        },
        Err(_) => {
            let mut image_digest_layerdigest_to_layerdiffid = HashMap::new();
            image_digest_layerdigest_to_layerdiffid.insert(layer_diff_id.clone(),layer_digest.clone());
            image_digest_json_value = ImageLayerLayerDigestToLayerDiffIDJSONValue {
                image_digest_layerdigest_to_layerdiffid
            };
        }
    };

    tree.insert(image_digest.clone(), &image_digest_json_value)?;
    // let value1 = tree
    //     .get(image_digest.clone())?
    //     .expect("Value not found")
    //     .decode()
    //     .expect("Decoding failed");
    // assert_eq!(value, image_repositories_json_value);
    // println!("image_digest:{:?}",value1);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}