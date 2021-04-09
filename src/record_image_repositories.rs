use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageVersionJSONValue };

pub async fn record_image_repositories(db: &sled::Db,image_name:String,image_version:String,image_digest:String) -> sled::Result<()> {
    let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
    let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
        db.open_tree("image_repositories")?,
    );

    let value = tree
    .get(image_name.clone());

    let  image_repositories_json_value:ImageVersionJSONValue;
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            let mut image_repositories = HashMap::new();
                            image_repositories.insert(image_name_version.clone(),image_digest.clone());
                            res2.image_version.insert(image_name.clone(),image_repositories);
                            image_repositories_json_value = ImageVersionJSONValue {
                                image_version: res2.image_version
                            };
                        }
                        _ => {
                            let mut result = HashMap::new();
                            let mut image_repositories = HashMap::new();
                            image_repositories.insert(image_name_version.clone(),image_digest.clone());
                            result.insert(image_name.clone(),image_repositories);
                            image_repositories_json_value = ImageVersionJSONValue {
                                image_version: result
                            };
                        }
                    }
                },
                _ => {
                        let mut result = HashMap::new();
                        let mut image_repositories = HashMap::new();
                        image_repositories.insert(image_name_version.clone(),image_digest.clone());
                        result.insert(image_name.clone(),image_repositories);
                        image_repositories_json_value = ImageVersionJSONValue {
                            image_version: result
                        };
                }
            }
        },
        Err(_) => {
                    let mut result = HashMap::new();
                    let mut image_repositories = HashMap::new();
                    image_repositories.insert(image_name_version.clone(),image_digest.clone());
                    result.insert(image_name.clone(),image_repositories);
                    image_repositories_json_value = ImageVersionJSONValue {
                        image_version: result
                    };
        }
    };

    tree.insert("Repositories".to_string(), &image_repositories_json_value)?;
    // let value1 = tree
    //     .get(image_name.clone())?
    //     .expect("Value not found")
    //     .decode()
    //     .expect("Decoding failed");
    // assert_eq!(value, image_repositories_json_value);
    // println!("image_version:{:?}",value1);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}