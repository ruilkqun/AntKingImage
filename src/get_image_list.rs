use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageVersionJSONValue,ImageListItemJSONValue,ImageSpecItemJSONValue };
use crate::public_struct::{ ImageDigestToNameVersionJSONValue };
use crate::get_image_size::get_image_size_repositories;


pub async fn read_image_list_repositories(db: &sled::Db,image_name:String,image_version:String,image_digest:String) -> Vec<ImageListItemJSONValue> {
    let mut result_vec:Vec<ImageListItemJSONValue> = Vec::new();
    if image_digest == "".to_string(){
        if image_name == "".to_string() {
        // TODO 遍历记录仓库所有镜像
            let tree_tmp = match db.open_tree("image_repositories"){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };
            let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
                tree_tmp
            );

            let value_1 = match tree.get("Repositories".to_string()){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value_2 = match value_1{
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value = match value_2.decode() {
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };


            for (image_name,image_digest_list) in value.image_version.iter(){
                for (image_name_version,image_digest) in image_digest_list.iter(){
                    let image_version_1 = image_name_version.split(":");
                    let image_version_2:Vec<&str> = image_version_1.collect();
                    let image_version = format!("{}",image_version_2[image_version_2.len() - 1]);
                    let image_digest = image_digest;

                    let size = get_image_size_repositories(db,image_digest.clone()).await;
                    let spec_item = ImageSpecItemJSONValue {
                        image_digest:image_digest.clone(),
                        annotations:HashMap::new()
                    };
                    let mut repo_tags = Vec::new();
                    let mut repo_digests = Vec::new();
                    repo_tags.push(image_version.clone());
                    repo_digests.push(image_name.clone());
                    let result_item = ImageListItemJSONValue{
                        id: image_digest.clone(),
                        repo_tags,
                        repo_digests,
                        size: size as u64,
                        uid: 1000,
                        username: "root".to_string(),
                        spec: spec_item
                    };
                    result_vec.push(result_item);
                }
            }
            return result_vec
        }else if image_version == "".to_string() {
        // TODO 遍历当前镜像名所有版本
            let tree_tmp = match db.open_tree("image_repositories"){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };
            let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
            tree_tmp
            );
            let value_1 = match tree.get("Repositories".to_string()){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value_2 = match value_1{
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value = match value_2.decode() {
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let image_digest_list = value.image_version[&image_name.clone()].clone();
            for (image_name_version,image_digest) in image_digest_list.iter(){
                let image_version_1 = image_name_version.split(":");
                let image_version_2:Vec<&str> = image_version_1.collect();
                let image_version = format!("{}",image_version_2[image_version_2.len() - 1]);
                let image_digest = image_digest;

                let size = get_image_size_repositories(db,image_digest.clone()).await;
                let spec_item = ImageSpecItemJSONValue {
                    image_digest:image_digest.clone(),
                    annotations:HashMap::new()
                };
                let mut repo_tags = Vec::new();
                let mut repo_digests = Vec::new();
                repo_tags.push(image_version.clone());
                repo_digests.push(image_name.clone());
                let result_item = ImageListItemJSONValue{
                    id: image_digest.clone(),
                    repo_tags,
                    repo_digests,
                    size: size as u64,
                    uid: 1000,
                    username: "root".to_string(),
                    spec: spec_item
                };
                result_vec.push(result_item);
            }
            return result_vec
        }else {
        // TODO 查找指定镜像版本的摘要
            let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
            let tree_tmp = match db.open_tree("image_repositories"){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };
            let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
            tree_tmp
            );
            let value_1 = match tree.get("Repositories".to_string()){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value_2 = match value_1{
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value = match value_2.decode() {
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let image_digest = format!("{}",value.image_version[&image_name.clone()][&image_name_version.clone()]);

            let tree_tmp = match db.open_tree("image_digest_name_version_repositories"){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };
            let tree = TreeWrapper::<JSONEncoder<ImageDigestToNameVersionJSONValue>, JSONEncoder<ImageDigestToNameVersionJSONValue>>::new(
            tree_tmp
            );

            let value_1 = match tree.get(image_digest.clone()){
                Ok(res) => res,
                Err(_) => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value_2 = match value_1{
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let value = match value_2.decode() {
                Some(res) => res,
                None => {
                    let result_item = ImageListItemJSONValue::default();
                    result_vec.push(result_item);
                    return result_vec
                }
            };

            let image_name_version_info = value.image_info[&image_digest.clone()].clone();
            let image_name = format!("{}",image_name_version_info[0]);
            let image_version = format!("{}",image_name_version_info[1]);

            let size = get_image_size_repositories(db,image_digest.clone()).await;
            let spec_item = ImageSpecItemJSONValue {
                image_digest:image_digest.clone(),
                annotations:HashMap::new()
            };

            let mut repo_tags = Vec::new();
            let mut repo_digests = Vec::new();
            repo_tags.push(image_version.clone());
            repo_digests.push(image_name.clone());
            let result_item = ImageListItemJSONValue{
                id: image_digest.clone(),
                repo_tags,
                repo_digests,
                size: size as u64,
                uid: 1000,
                username: "root".to_string(),
                spec: spec_item
            };
            result_vec.push(result_item);
            return result_vec
        }
    }else {
    //  TODO 摘要不为空，直接计算size
        let tree_tmp = match db.open_tree("image_digest_name_version_repositories"){
            Ok(res) => res,
            Err(_) => {
                let result_item = ImageListItemJSONValue::default();
                result_vec.push(result_item);
                return result_vec
            }
        };

        let tree = TreeWrapper::<JSONEncoder<ImageDigestToNameVersionJSONValue>, JSONEncoder<ImageDigestToNameVersionJSONValue>>::new(
        tree_tmp
        );

        let value_1 = match tree.get(image_digest.clone()){
            Ok(res) => res,
            Err(_) => {
                let result_item = ImageListItemJSONValue::default();
                result_vec.push(result_item);
                return result_vec
            }
        };

        let value_2 = match value_1{
            Some(res) => res,
            None => {
                let result_item = ImageListItemJSONValue::default();
                result_vec.push(result_item);
                return result_vec
            }
        };

        let value = match value_2.decode() {
            Some(res) => res,
            None => {
                let result_item = ImageListItemJSONValue::default();
                result_vec.push(result_item);
                return result_vec
            }
        };

        let image_name_version_info = value.image_info[&image_digest.clone()].clone();
        let image_name = format!("{}",image_name_version_info[0]);
        let image_version = format!("{}",image_name_version_info[1]);

        let size = get_image_size_repositories(db,image_digest.clone()).await;
        let spec_item = ImageSpecItemJSONValue {
            image_digest:image_digest.clone(),
            annotations:HashMap::new()
        };

        let mut repo_tags = Vec::new();
        let mut repo_digests = Vec::new();
        repo_tags.push(image_version.clone());
        repo_digests.push(image_name.clone());
        let result_item = ImageListItemJSONValue{
            id: image_digest.clone(),
            repo_tags,
            repo_digests,
            size: size as u64,
            uid: 1000,
            username: "root".to_string(),
            spec: spec_item
        };
        result_vec.push(result_item);
        return result_vec
    }
}