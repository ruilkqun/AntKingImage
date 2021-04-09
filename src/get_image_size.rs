#![allow(unused_assignments)]
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::{ ImageChainIDJSONValue };


pub async fn get_image_size_repositories(db: &sled::Db,image_digest:String) -> i32 {
    let tree_tmp = match db.open_tree("image_chain_id"){
        Ok(res) => res,
        Err(_) => {
            return 1
        }
    };
    let tree = TreeWrapper::<JSONEncoder<ImageChainIDJSONValue>, JSONEncoder<ImageChainIDJSONValue>>::new(
        tree_tmp
    );


    let value_1 = match tree.get(image_digest.clone()){
        Ok(res) => res,
        Err(_) => {
            return 1
        }
    };

    let value_2 = match value_1{
        Some(res) => res,
        None => {
            return 1
        }
    };

    let value = match value_2.decode() {
        Some(res) => res,
        None => {
           return 1
        }
    };


    let chain_list = value.image_chain_id;
    let mut size = 0;
    for (_k,v) in chain_list.iter() {
        size += v.size.parse::<i32>().unwrap();
    }

    return size
}