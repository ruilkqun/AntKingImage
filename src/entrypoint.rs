use crate::get_manifest::get_manifest_info;
use crate::local_repositories::judge_image_local;
use crate::get_config::{ write_config_json,read_config_json };
use crate::record_image_layer_level::record_image_layer_diff_id_to_level;
use crate::utils::{determine_whether_image_layer_exists,compute_layer_size,computer_layer_chain_id};
use crate::record_image_digest_layerdiffid_layerdigest::{record_image_digest_layer_diff_id_to_layer_digest,record_image_digest_layer_digest_layer_diff_id};
use crate::get_layers::get_layers;
use crate::record_image_repositories::record_image_repositories;
use crate::record_image_chainid::record_image_chain_id;

pub async fn pull_image(db: &sled::Db,repositories_url_ip:String,image_name:String,image_version:String,username:String,password:String) {
    // 获取manifest
    let manifest_info = get_manifest_info(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone()).await;
    let manifest_info_1 = match manifest_info {
        Ok(res) => {
            res
        },
        Err(e) => {
            println!("Get manifest_ Info failed!Reason:{}",e);
            return
        }
    };


    let image_digest = manifest_info_1.config.digest.clone();
    let image_digest_no_sha256_1 = image_digest.split(':');
    let image_digest_no_sha256_2: Vec<&str> = image_digest_no_sha256_1.collect();
    let image_digest_no_sha256_3 = image_digest_no_sha256_2[1];
    let image_digest_no_sha256 = format!("{}",image_digest_no_sha256_3.clone());


    // let image_digest = get_config_digest(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone()).await;
    // 判断本地镜像是否存在
    let local_image = judge_image_local(db, image_name.clone(), image_version.clone(), image_digest.clone()).await;
    if local_image == true {
        println!("Image already exists!");
        return
    } else {
        // 获取镜像配置文件
        write_config_json(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_digest.clone()).await;
        // 读配置文件 在sled记录镜像diff_id和level对应关系
        let config_info = read_config_json(image_digest_no_sha256.clone()).await;
        let layer_1 = format!("{}", config_info["rootfs"]["diff_ids"]);
        let layer_2 = layer_1.split(',');
        let layer_3: Vec<&str> = layer_2.collect();

        let mut layer_parent_chain_id = "".to_string();
        for i in 0..layer_3.len() {
            let layer_diff_id1 = format!("{}", config_info["rootfs"]["diff_ids"][i]);
            let layer_diff_id2 = layer_diff_id1.split('"');
            let layer_diff_id3: Vec<&str> = layer_diff_id2.collect();
            let layer_diff_id = format!("{}", layer_diff_id3[1]);
            record_image_layer_diff_id_to_level(db, image_digest_no_sha256.clone(), layer_diff_id.clone(), i as i64).await.unwrap();
            // 判断 本地layer层是否存在
            let whether_image_layer = determine_whether_image_layer_exists(db,image_digest_no_sha256.clone(),layer_diff_id.clone());

            let layer_digest = manifest_info_1.layers[i].digest.clone();
            let layer_digest1 = layer_digest.split(':');
            let layer_digest2: Vec<&str> = layer_digest1.collect();
            let layer_digest1_no_sha256 = format!("{}", layer_digest2[1]);
            // let layer_digest = get_layer_digest(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone(),i).await;

            if whether_image_layer{
                continue
            }else {
                record_image_digest_layer_diff_id_to_layer_digest(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();
                record_image_digest_layer_digest_layer_diff_id(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();
                get_layers(repositories_url_ip.clone(),username.clone(), password.clone(),image_name.clone(),image_digest.clone(),layer_digest.clone(),layer_diff_id.clone()).await;

                // 记录chain_id
                let chain_id = computer_layer_chain_id(layer_parent_chain_id.clone(),layer_diff_id.clone());
                let path = format!("/var/lib/AntKing/images/{}/{}.tar",image_digest_no_sha256.clone(),layer_digest1_no_sha256.clone());
                println!("path:{}",path);
                let size = compute_layer_size(path.clone());
                record_image_chain_id(db,image_digest_no_sha256.clone(),chain_id.clone(),layer_diff_id.clone(),layer_diff_id.clone(),layer_parent_chain_id.clone(),size).await.unwrap();
                layer_parent_chain_id = chain_id.clone();
            }
        }
        record_image_repositories(db,image_name.clone(),image_version.clone(),image_digest.clone()).await.unwrap();
    }
}