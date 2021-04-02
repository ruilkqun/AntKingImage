use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageLayerLevelJSONValue };



pub async fn record_image_layer_diff_id_to_level(db: &sled::Db,image_digest:String,layer_diff_id:String,layer_level:i64) -> sled::Result<()> {
    let tree = TreeWrapper::<JSONEncoder<ImageLayerLevelJSONValue>, JSONEncoder<ImageLayerLevelJSONValue>>::new(
        db.open_tree("image_layer_level")?,
    );

    let value = tree
    .get(image_digest.clone());

    let layer_level = format!("{}",layer_level);

    let mut image_digest_json_value = ImageLayerLevelJSONValue::default();
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            res2.image_layer_level.insert(layer_diff_id.clone(),layer_level.clone());
                            image_digest_json_value = ImageLayerLevelJSONValue {
                                image_layer_level: res2.image_layer_level
                            };
                        }
                        _ => {
                            let mut image_layer_level = HashMap::new();
                            image_layer_level.insert(layer_diff_id.clone(),layer_level.clone());
                            image_digest_json_value = ImageLayerLevelJSONValue {
                                image_layer_level
                            };
                        }
                    }
                },
                _ => {
                        let mut image_layer_level = HashMap::new();
                        image_layer_level.insert(layer_diff_id.clone(),layer_level.clone());
                        image_digest_json_value = ImageLayerLevelJSONValue {
                            image_layer_level
                        };
                }
            }
        },
        Err(_) => {
                    let mut image_layer_level = HashMap::new();
                    image_layer_level.insert(layer_diff_id.clone(),layer_level.clone());
                    image_digest_json_value = ImageLayerLevelJSONValue {
                        image_layer_level
                    };
        }
    };

    tree.insert(image_digest.clone(), &image_digest_json_value)?;
    let value1 = tree
        .get(image_digest.clone())?
        .expect("Value not found")
        .decode()
        .expect("Decoding failed");
    // assert_eq!(value, image_repositories_json_value);
    println!("image_digest:{:?}",value1);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}
