use crate::get_manifest::{get_manifest_info,Manifest as NDockerManifest};
use crate::local_repositories::judge_image_local;
use crate::get_config::{ write_config_json,read_config_json };
use crate::record_image_layer_level::record_image_layer_diff_id_to_level;
use crate::utils::{ determine_whether_image_layer_exists,compute_layer_size,computer_layer_chain_id };
use crate::record_image_digest_layerdiffid_layerdigest::{ record_image_digest_layer_diff_id_to_layer_digest,record_image_digest_layer_digest_layer_diff_id };
use crate::get_layers::get_layers;
use crate::record_image_repositories::record_image_repositories;
use crate::record_image_chainid::record_image_chain_id;
use crate::get_manifest_dockerhub::{get_manifest_info_dockerhub,Manifest as DockerManifest};
use crate::get_config_dockerhub::{ write_config_json_dockerhub,read_config_json_dockerhub };
use crate::get_layers_dockerhub::get_layers_dockerhub;
use crate::get_token_dockerhub::get_token_dockerhub;
// use rayon::prelude::*;
use std::collections::HashMap;


pub async fn pull_image(db: &sled::Db,repositories_url_ip:String,image_name:String,image_version:String,username:String,password:String,docker:bool) {
    // 获取manifest(非Docker OCI镜像和Docker OCI镜像)

    // 获取dockerhub token
    let  token;
    match !docker {
        true => {
            token = "".to_string();
        },
        false => {
            let token_1 = get_token_dockerhub(image_name.clone()).await.unwrap();
            token = format!("{}",token_1.access_token);
        }
    }

    let mut manifest_info1:NDockerManifest = NDockerManifest::default();
    let mut manifest_info_docker1:DockerManifest = DockerManifest::default();
    match !docker{
        true => {
            let manifest_info = get_manifest_info(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone()).await;
            manifest_info1 = match manifest_info {
                Ok(res) => {
                    res
                },
                Err(e) => {
                    println!("Get manifest_Info failed!Reason:{}",e);
                    return
                }
            }
        },
        false => {
            let manifest_info_docker = get_manifest_info_dockerhub( image_name.clone(), image_version.clone(),token.clone()).await;
            manifest_info_docker1 = match manifest_info_docker {
                Ok(res) => {
                    res
                },
                Err(e) => {
                    println!("Get manifest_Info failed!Reason:{}",e);
                    return
                }
            }
        }
    };


    // 获取不带sha256前缀的镜像摘要
    let image_digest;
    match !docker {
        true => image_digest = manifest_info1.config.digest.clone(),
        false => image_digest = manifest_info_docker1.config.digest.clone()
    }

    let image_digest_no_sha256_1 = image_digest.split(':');
    let image_digest_no_sha256_2: Vec<&str> = image_digest_no_sha256_1.collect();
    let image_digest_no_sha256_3 = image_digest_no_sha256_2[1];
    let image_digest_no_sha256 = format!("{}",image_digest_no_sha256_3.clone());


    // 判断本地镜像是否存在
    let local_image = judge_image_local(db, image_name.clone(), image_version.clone(), image_digest.clone()).await;
    if local_image {
        println!("Image already exists!");
        return
    } else {
        match !docker {
            true => {
                // 获取镜像配置文件
                write_config_json(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_digest.clone()).await;
            },
            false => {
                // 获取镜像配置文件
                write_config_json_dockerhub( image_name.clone(), image_digest.clone(),token.clone()).await;
            }
        };

        let config_info = match !docker {
            true =>   read_config_json(image_digest_no_sha256.clone()).await,
            false =>  read_config_json_dockerhub(image_digest_no_sha256.clone()).await
        };

        // 读配置文件 在sled记录镜像diff_id和level对应关系
        let layer_1 = format!("{}", config_info["rootfs"]["diff_ids"]);
        let layer_2 = layer_1.split(',');
        let layer_3: Vec<&str> = layer_2.collect();
        // 最初为并行处理，现为方便处理各个字段
        let mut rayon_vec = Vec::new();
        // 父chain_id
        let mut layer_parent_chain_id = "".to_string();
        for i in 0..layer_3.len() {
            // [image_digest_no_sha256,layer_digest_no_sha256,layer_diff_id,chain_id,parent_chain_id]
            let mut layer_vec = Vec::new();
            // 获取当前层的diff_id
            let layer_diff_id1 = format!("{}", config_info["rootfs"]["diff_ids"][i]);
            let layer_diff_id2 = layer_diff_id1.split('"');
            let layer_diff_id3: Vec<&str> = layer_diff_id2.collect();
            let layer_diff_id = format!("{}", layer_diff_id3[1]);
            record_image_layer_diff_id_to_level(db, image_digest_no_sha256.clone(), layer_diff_id.clone(), i as i64).await.unwrap();

            // 获取当前层层摘要
            let layer_digest;
            match !docker {
                true => layer_digest = manifest_info1.layers[i].digest.clone(),
                false => layer_digest = manifest_info_docker1.layers[i].digest.clone()
            }

            let layer_digest1 = layer_digest.split(':');
            let layer_digest2: Vec<&str> = layer_digest1.collect();
            let layer_digest1_no_sha256 = format!("{}", layer_digest2[1]);

            layer_vec.push(image_digest_no_sha256.clone());
            layer_vec.push(layer_digest1_no_sha256.clone());
            layer_vec.push(layer_diff_id.clone());

            // 判断 本地layer层是否存在标记
            let whether_image_layer = determine_whether_image_layer_exists(db,image_digest_no_sha256.clone(),layer_diff_id.clone());
            if whether_image_layer{
                continue
            }else {
                record_image_digest_layer_diff_id_to_layer_digest(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();
                record_image_digest_layer_digest_layer_diff_id(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();

                let chain_id = computer_layer_chain_id(layer_parent_chain_id.clone(),layer_diff_id.clone());
                layer_vec.push(chain_id.clone());
                layer_vec.push(layer_parent_chain_id.clone());
                layer_parent_chain_id = chain_id.clone();
            }

            layer_vec.push(image_digest.clone());
            layer_vec.push(layer_digest.clone());
            let mut layer_hashmap = HashMap::new();
            layer_hashmap.insert("item",layer_vec);
            rayon_vec.push(layer_hashmap);
        }


        for item in rayon_vec {
            match !docker {
                true => {
                    // 获取镜像层
                    get_layers(
                        repositories_url_ip.clone(),
                        username.clone(),
                        password.clone(),
                        image_name.clone(),
                        item["item"][5].clone(),
                        item["item"][6].clone(),
                        item["item"][2].clone()
                    ).await;
                },
                false => {
                    // 获取镜像层
                    get_layers_dockerhub(
                        image_name.clone(),
                        item["item"][5].clone(),
                        item["item"][6].clone(),
                        item["item"][2].clone(),
                        token.clone()
                    ).await;
                }
            };
            // 计算层size
            let path = format!("/var/lib/AntKing/gz/{}/{}.tar", item["item"][0.clone()], item["item"][1].clone());
            let size = compute_layer_size(path.clone());
            // 记录chain_id
            record_image_chain_id(db, item["item"][0].clone(), item["item"][3].clone(), item["item"][2].clone(), item["item"][2].clone(), item["item"][4].clone(), size).await.unwrap();
        }
        // 数据库记录镜像
        let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
        println!("Download Image {} complete!",image_name_version);
        record_image_repositories(db,image_name.clone(),image_version.clone(),image_digest.clone()).await.unwrap();
    }
}