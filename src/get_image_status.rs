use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageListItemJSONValue,ImageSpecItemJSONValue };
use crate::public_struct::{ ImageDigestToNameVersionJSONValue };
use crate::get_image_size::get_image_size_repositories;
use crate::utils::{ create_sled_db,get_completed_digest };


pub async fn read_image_status_repositories(image_digest_no_sha256:String) -> ImageListItemJSONValue {
    let image_completed_digest_no_sha256 = get_completed_digest(image_digest_no_sha256.clone());
    let image_digest = format!("sha256:{}",image_completed_digest_no_sha256.clone());
    // println!("image_digest:{}",image_digest.clone());
    // let image_digest1 = "sha256:519e12e2a84a9eb18094635ae1edfd97b26f95dbc66e317eefb657a1cb08c8dc".to_string();
    // println!("image_digest:{}",image_digest.clone());
    // println!("image_digest1:{}",image_digest1.clone());
    // if image_digest.clone() == image_digest1.clone() {
    //     println!("hi")
    // }else {
    //     println!("ha")
    // }
    let db_tmp = create_sled_db().await;
    let db = match db_tmp{
      Some(res) => res,
      None => {
            let result_item = ImageListItemJSONValue::default();
            return result_item
      }
    };


    let tree_tmp = match db.open_tree("image_digest_name_version_repositories"){
        Ok(res) => res,
        Err(_) => {
            let result_item = ImageListItemJSONValue::default();
            return result_item
        }
    };

    let tree = TreeWrapper::<JSONEncoder<ImageDigestToNameVersionJSONValue>, JSONEncoder<ImageDigestToNameVersionJSONValue>>::new(
    tree_tmp
    );

    // println!("image_digest:{}",image_digest.clone());
    let value_1 = match tree.get(image_digest.clone()){
        Ok(res) => res,
        Err(_) => {
            let result_item = ImageListItemJSONValue::default();
            return result_item
        }
    };

    let value_2 = match value_1{
        Some(res) => res,
        None => {
            let result_item = ImageListItemJSONValue::default();
            return result_item
        }
    };

    let value = match value_2.decode() {
        Some(res) => res,
        None => {
            let result_item = ImageListItemJSONValue::default();
            return result_item
        }
    };

    let image_name_version_info = value.image_info[&image_digest.clone()].clone();
    let image_name = format!("{}",image_name_version_info[0]);
    let image_version = format!("{}",image_name_version_info[1]);
    let image_name_version= format!("{}:{}",image_name,image_version);


    // 获取镜像大小 镜像摘要 不要带sha256
    let size = get_image_size_repositories(&db,image_completed_digest_no_sha256.clone()).await;
    let spec_item = ImageSpecItemJSONValue {
        image_digest:image_digest.clone(),
        annotations:HashMap::new()
    };

    let mut repo_tags = Vec::new();
    let repo_digests = Vec::new();

    repo_tags.push(image_name_version.clone());
    let result_item = ImageListItemJSONValue{
        id: image_digest.clone(),
        repo_tags,
        repo_digests,
        size: size as u64,
        uid: 1000,
        username: "root".to_string(),
        spec: spec_item
    };
    return result_item
}