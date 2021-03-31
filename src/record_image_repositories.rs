use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageVersionJSONValue };

pub fn record_image_repositories(db: &sled::Db,image_name:String,image_version:String,image_digest:String) -> sled::Result<()> {
    let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
    let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
        db.open_tree("image_repositories")?,
    );

    let value = tree
    .get(image_name.clone());

    let mut image_repositories_json_value:ImageVersionJSONValue = ImageVersionJSONValue::default();
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            res2.image_version.insert(image_name_version.clone(),image_digest.clone());
                            image_repositories_json_value = ImageVersionJSONValue {
                                image_version: res2.image_version
                            };
                        }
                        _ => {
                            let mut image_repositories = HashMap::new();
                            image_repositories.insert(image_name_version.clone(),image_digest.clone());
                            image_repositories_json_value = ImageVersionJSONValue {
                                image_version: image_repositories
                            };
                        }
                    }
                },
                _ => {
                    let mut image_repositories = HashMap::new();
                    image_repositories.insert(image_name_version.clone(),image_digest.clone());
                    image_repositories_json_value = ImageVersionJSONValue {
                        image_version: image_repositories
                    };
                }
            }
        },
        Err(_) => {
            let mut image_repositories = HashMap::new();
            image_repositories.insert(image_name_version.clone(),image_digest.clone());
            image_repositories_json_value = ImageVersionJSONValue {
                image_version: image_repositories
            };
        }
    };

    tree.insert(image_name.clone(), &image_repositories_json_value)?;
    let value1 = tree
        .get(image_name.clone())?
        .expect("Value not found")
        .decode()
        .expect("Decoding failed");
    // assert_eq!(value, image_repositories_json_value);
    println!("image_version:{:?}",value1);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}