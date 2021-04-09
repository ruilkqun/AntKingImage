use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageDigestToNameVersionJSONValue };

pub async fn record_image_digest_image_name_version_repositories(db: &sled::Db,image_name:String,image_version:String,image_digest:String) -> sled::Result<()> {
    let tree = TreeWrapper::<JSONEncoder<ImageDigestToNameVersionJSONValue>, JSONEncoder<ImageDigestToNameVersionJSONValue>>::new(
        db.open_tree("image_digest_name_version_repositories")?,
    );

    let value = tree
    .get(image_name.clone());

    let  image_repositories_json_value:ImageDigestToNameVersionJSONValue;
    match value {
        Ok(res) => {
            match res {
                Some(res1) =>{
                    match res1.decode() {
                        Some(mut res2) => {
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(image_name.clone());
                            tmp_vec.push(image_version.clone());
                            res2.image_info.insert(image_digest.clone(),tmp_vec);
                            image_repositories_json_value = ImageDigestToNameVersionJSONValue {
                                image_info: res2.image_info
                            };
                        }
                        _ => {
                            let mut image_repositories = HashMap::new();
                            let mut tmp_vec = Vec::new();
                            tmp_vec.push(image_name.clone());
                            tmp_vec.push(image_version.clone());
                            image_repositories.insert(image_digest.clone(),tmp_vec);
                            image_repositories_json_value = ImageDigestToNameVersionJSONValue {
                                image_info: image_repositories
                            };
                        }
                    }
                },
                _ => {
                        let mut image_repositories = HashMap::new();
                        let mut tmp_vec = Vec::new();
                        tmp_vec.push(image_name.clone());
                        tmp_vec.push(image_version.clone());
                        image_repositories.insert(image_digest.clone(),tmp_vec);
                        image_repositories_json_value = ImageDigestToNameVersionJSONValue {
                            image_info: image_repositories
                        };
                }
            }
        },
        Err(_) => {
                    let mut image_repositories = HashMap::new();
                    let mut tmp_vec = Vec::new();
                    tmp_vec.push(image_name.clone());
                    tmp_vec.push(image_version.clone());
                    image_repositories.insert(image_digest.clone(),tmp_vec);
                    image_repositories_json_value = ImageDigestToNameVersionJSONValue {
                        image_info: image_repositories
                    };
        }
    };

    tree.insert(image_digest.clone(), &image_repositories_json_value)?;
    Ok(())
}