use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageChainIDJSONValue,ImageChainIDItemJSONValue };


pub async fn record_image_chain_id(db: &sled::Db,image_digest:String,image_chain_id:String,image_cache_id:String,image_diff_id:String,image_parent_chain_id:String,image_size:String) -> sled::Result<()> {
    let tree = TreeWrapper::<JSONEncoder<ImageChainIDJSONValue>, JSONEncoder<ImageChainIDJSONValue>>::new(
        db.open_tree("image_chain_id")?,
    );

    let value = tree
    .get(image_digest.clone());

    let mut image_digest_json_value = ImageChainIDJSONValue{
        image_chain_id: HashMap::new()
    };
    let mut image_chain_id_item = ImageChainIDItemJSONValue::default();
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            image_chain_id_item.cache_id = image_cache_id.clone();
                            image_chain_id_item.diff_id = image_diff_id.clone();
                            image_chain_id_item.parent_chain_id = image_parent_chain_id.clone();
                            image_chain_id_item.size = image_size.clone();
                            res2.image_chain_id.insert(image_chain_id.clone(),image_chain_id_item);
                            image_digest_json_value = ImageChainIDJSONValue {
                                image_chain_id: res2.image_chain_id
                            };
                        }
                        _ => {
                            image_chain_id_item.cache_id = image_cache_id.clone();
                            image_chain_id_item.diff_id = image_diff_id.clone();
                            image_chain_id_item.parent_chain_id = image_parent_chain_id.clone();
                            image_chain_id_item.size = image_size.clone();
                            let mut image_chain_id_1 = HashMap::new();
                            image_chain_id_1.insert(image_chain_id.clone(),image_chain_id_item);
                            image_digest_json_value = ImageChainIDJSONValue {
                                image_chain_id:image_chain_id_1
                            };
                        }
                    }
                },
                _ => {
                        image_chain_id_item.cache_id = image_cache_id.clone();
                        image_chain_id_item.diff_id = image_diff_id.clone();
                        image_chain_id_item.parent_chain_id = image_parent_chain_id.clone();
                        image_chain_id_item.size = image_size.clone();
                        let mut image_chain_id_1 = HashMap::new();
                        image_chain_id_1.insert(image_chain_id.clone(),image_chain_id_item);
                        image_digest_json_value = ImageChainIDJSONValue {
                            image_chain_id:image_chain_id_1
                        };
                }
            }
        },
        Err(_) => {
                    image_chain_id_item.cache_id = image_cache_id.clone();
                    image_chain_id_item.diff_id = image_diff_id.clone();
                    image_chain_id_item.parent_chain_id = image_parent_chain_id.clone();
                    image_chain_id_item.size = image_size.clone();
                    let mut image_chain_id_1 = HashMap::new();
                    image_chain_id_1.insert(image_chain_id.clone(),image_chain_id_item);
                    image_digest_json_value = ImageChainIDJSONValue {
                        image_chain_id:image_chain_id_1
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