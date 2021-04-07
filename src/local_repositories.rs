use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageVersionJSONValue };
use crate::utils::create_sled_db;


pub async fn judge_image_local(db: &sled::Db,image_name:String,image_version:String,image_digest:String) -> bool{
    let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
    let tree_tmp = match db.open_tree("image_repositories") {
        sled::Result::Ok(res) => {
            res
        },
        _ => {
            return false
        }
    };
    let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
        tree_tmp,
    );

    let value = tree
    .get(image_name.clone());

    match value {
        Ok(res) => {
            match res  {
                Some(res1) => {
                    match res1.decode() {
                        None => false,
                        Some(res2) => {
                            let search_result = res2.image_version[image_name_version.as_str()].clone();
                            // println!("search_result:{}",search_result.clone());
                            // println!("image_digest:{}",image_digest.clone());
                            if search_result == image_digest {
                                true
                            }else {
                                false
                            }
                        }
                    }
                }
                _ => false
            }
        },
        _ => false
    }

}


pub async fn get_image_digest_local(image_name:String,image_version:String) -> Result<String, ()> {
    let db_tmp = create_sled_db().await;
    let db = match db_tmp{
      Some(res) => res,
      None => return Ok("".to_string())
    };

    let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
    let tree_tmp = match db.open_tree("image_repositories") {
        sled::Result::Ok(res) => {
            res
        },
        _ => {
            return Ok("".to_string())
        }
    };
    let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
        tree_tmp,
    );

    let value = tree
    .get(image_name.clone());

    match value {
        Ok(res) => {
            match res  {
                Some(res1) => {
                    match res1.decode() {
                        None => Ok("".to_string()),
                        Some(res2) => {
                            let search_result = res2.image_version[image_name_version.as_str()].clone();
                            Ok(search_result)
                        }
                    }
                }
                _ => Ok("".to_string())
            }
        },
        _ => Ok("".to_string())
    }

}