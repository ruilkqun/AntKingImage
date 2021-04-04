use crate::get_manifest::get_manifest_info;
use crate::local_repositories::judge_image_local;
use crate::get_config::{ write_config_json,read_config_json };
use crate::record_image_layer_level::record_image_layer_diff_id_to_level;
use crate::utils::{determine_whether_image_layer_exists,compute_layer_size,computer_layer_chain_id};
use crate::record_image_digest_layerdiffid_layerdigest::{record_image_digest_layer_diff_id_to_layer_digest,record_image_digest_layer_digest_layer_diff_id};
use crate::get_layers::get_layers;
use crate::record_image_repositories::record_image_repositories;
use crate::record_image_chainid::record_image_chain_id;

use std::cmp::min;
use indicatif::{ProgressBar, ProgressStyle};


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

            let layer_size = manifest_info_1.layers[i].size.clone();
            // let layer_digest = get_layer_digest(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone(),i).await;

            if whether_image_layer{
                continue
            }else {
                record_image_digest_layer_diff_id_to_layer_digest(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();
                record_image_digest_layer_digest_layer_diff_id(db,image_digest_no_sha256.clone(),layer_diff_id.clone(),layer_digest.clone()).await.unwrap();


                let layer_digest_progress = layer_digest1_no_sha256.clone();
                let path_progress = format!("du -shb /var/lib/AntKing/images/{}/* | grep {} || du -shb /var/lib/AntKing/gz/{}/* | grep {}",image_digest_no_sha256.clone(),layer_digest1_no_sha256.clone(),image_digest_no_sha256.clone(),layer_digest1_no_sha256.clone());

                // let path_progress = format!("/var/lib/AntKing/gz/{}/{}.tar.gz",image_digest_no_sha256.clone(),layer_digest1_no_sha256.clone());

                let handle = std::thread::spawn(move || {
                    // println!("hi");
                    // std::fs::File::create(path_progress.clone()).unwrap();
                    let mut downloaded = 0;
                    let total_size = layer_size as u64;

                    let pb = ProgressBar::new(total_size);
                    pb.set_style(ProgressStyle::default_bar()
                        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                        .progress_chars("=>-"));

                    // let mut i =0;
                    while downloaded < total_size {
                        // let cmd = format!("du -shb {}",path_progress.clone());
                        // println!("cmd:{}",cmd);
                        let cmd = format!("{}",path_progress.clone());
                        let result = String::from_utf8(std::process::Command::new("sh").arg("-c").arg(cmd).output().unwrap().stdout).unwrap();
                        if result != "".to_string() {
                            let result1 = result.split('/');
                            let result2:Vec<&str> = result1.collect();
                            let result3 = result2[0];
                            let result4 = format!("{}",result3.trim());
                            // println!("result:{}123",result4.clone());
                            // downloaded = compute_layer_size(path_progress.clone()).parse().unwrap();
                            // println!("downloaded:{}",downloaded);

                            downloaded = result4.parse::<u64>().unwrap();
                            // downloaded += i;
                            // i += 100;
                        }else {
                            downloaded = 0;
                        }

                        // downloaded = match result7.parse() {
                        //     Ok(res) => res,
                        //     Err(_) => 0
                        // };
                        // downloaded = 1;
                        let new = min(downloaded, total_size);
                        downloaded = new;
                        pb.set_position(new);
                    }

                    let message = format!("{} downloaded",layer_digest_progress.clone());
                    pb.finish_with_message(&*message.clone());
                            // 进度条
                            // progress_bar(layer_size.clone(), path_progress.clone(), layer_digest_progress).await;
                });


                // 获取镜像层
                get_layers(repositories_url_ip.clone(),username.clone(), password.clone(),image_name.clone(),image_digest.clone(),layer_digest.clone(),layer_diff_id.clone()).await;

                handle.join().unwrap();
                // 记录chain_id
                let chain_id = computer_layer_chain_id(layer_parent_chain_id.clone(),layer_diff_id.clone());
                let path = format!("/var/lib/AntKing/images/{}/{}.tar",image_digest_no_sha256.clone(),layer_digest1_no_sha256.clone());
                // println!("path:{}",path);
                let size = compute_layer_size(path.clone());
                record_image_chain_id(db,image_digest_no_sha256.clone(),chain_id.clone(),layer_diff_id.clone(),layer_diff_id.clone(),layer_parent_chain_id.clone(),size).await.unwrap();
                layer_parent_chain_id = chain_id.clone();
            }
        }
        let image_name_version = format!("{}:{}",image_name.clone(),image_version.clone());
        println!("Download Image {} complete!",image_name_version);
        record_image_repositories(db,image_name.clone(),image_version.clone(),image_digest.clone()).await.unwrap();
    }
}