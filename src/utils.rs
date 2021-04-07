use std::process::Command;
use std::fs;
use crate::sled_json::{ TreeWrapper, JSONEncoder };
use crate::public_struct::ImageLayerLayerDiffIDToLayerDigestJSONValue;

use std::cmp::min;
use indicatif::{ProgressBar, ProgressStyle};

use sled::{Config, Mode, Db};

pub async fn create_sled_db() -> Option<Db> {
    let db = Config::new()
        .mode(Mode::HighThroughput)
        .path("/var/lib/AntKing/db")
        .open();
    return match db {
        Ok(res) => {
            Some(res)
        },
        Err(e) => {
            println!("Failed to create database because:{}", e);
            None
        }
    }
}


pub async fn progress_bar(total_size:i64, path:String, layer_digest:String) {
    // println!("path:{}",path.clone());
    let mut downloaded = 0;
    let total_size = total_size as u64;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while downloaded < total_size {
        downloaded = compute_layer_size(path.clone()).parse().unwrap();
        let new = min(downloaded, total_size);
        downloaded = new;
        pb.set_position(new);
    }

    let message = format!("{} downloaded",layer_digest.clone());
    pb.finish_with_message(&*message.clone());
}


pub fn determine_whether_image_layer_exists(db: &sled::Db,image_digest:String,layer_diff_id:String) -> bool {
    let tree = TreeWrapper::<JSONEncoder<ImageLayerLayerDiffIDToLayerDigestJSONValue>, JSONEncoder<ImageLayerLayerDiffIDToLayerDigestJSONValue>>::new(
        db.open_tree("image_digest_layerdiffid_to_layerdigest").unwrap(),
    );

    let value = tree
    .get(image_digest.clone());
    match value{
        Ok(res) => {
            match res {
                Some(res1) => {
                    match res1.decode() {
                        None => false,
                        Some(res2) => {
                            match res2.image_digest_layerdiffid_to_layerdigest.get(&*layer_diff_id.clone()){
                                None => false,
                                Some(_) => true
                            }
                        }
                    }
                }

                _ => false
            }
        }
        _ => false
    }
}



pub fn compute_layer_size(path:String) -> String{
    // println!("path:{}",path.clone());
    // fs::File::create(path.clone()).unwrap();
    let content = fs::read(path.clone()).unwrap();
    let size = content.len();

    let cmd = format!("rm -rf {}",path);
    Command::new("sh").arg("-c").arg(cmd.clone()).output().unwrap();
    return format!("{}",size)
}


pub fn computer_layer_chain_id(layer_parent_chain_id:String,layer_diff_id:String) -> String {
    if layer_parent_chain_id != "".to_string() {
        let sha256_params = format!("echo -n \"sha256:{} sha256:{}\" | sha256sum",layer_parent_chain_id,layer_diff_id);
        let output1 = Command::new("sh").arg("-c").arg(sha256_params.clone()).output();
            match output1 {
                Ok(res) => {
                    match String::from_utf8(res.stdout){
                        Ok(res1) => {
                            let res1_1 = res1.split(' ');
                            let res1_2: Vec<&str> = res1_1.collect();
                            return format!("{}",res1_2[0])
                        },
                        Err(e) => {
                             println!("Execute sha256sum exception! Reason：{}",e);
                            "".to_string()
                        }
                    }
                }
                Err(e) => {
                    println!("Execute sha256sum exception! Reason：{}",e);
                    "".to_string()
                }
            }
        }else {
           layer_diff_id
        }
}