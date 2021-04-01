use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use sha256::digest_bytes;
use std::process::Command;


pub async fn get_layers(repositories_url_ip:String,image_name:String,image_digest:String,layer_digest:String)  {
    let url = format!("{}/v2/{}/blobs/{}", repositories_url_ip, image_name, layer_digest);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.image.rootfs.diff.tar.gzip".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());

    let image_storage_path_1 = image_digest.split(':');
    let image_storage_path_2: Vec<&str> = image_storage_path_1.collect();
    let image_storage_path_3 = image_storage_path_2[1];
    let image_storage_path = format!("{}",image_storage_path_3.clone());
    fs::create_dir_all(image_storage_path.clone()).unwrap();

    let layer_name_tmp = layer_digest.split(':');
    let layer_name: Vec<&str> = layer_name_tmp.collect();


    let path = format!("{}/{}.tar.gz",image_storage_path_3.clone(),layer_name[1]);
    let path1 = format!("{}/{}.tar",image_storage_path_3.clone(),layer_name[1]);

    match fs::remove_file(path1.clone()) {
        Ok(()) => {
            println!("Delete previous image layer file successfully！")
        },
        Err(e) => {
            println!("Failed to delete previous image layer file! Reason：{}",e)
        }
    }
    let mut file = File::create(path.clone()).unwrap();


    match client.get(url).basic_auth("admin",Some("saodiseng")).headers(headers).send().await {
        Ok(r) => {
            match r.bytes().await {
                Ok(r1) => {
                    let write_result = file.write(&*r1);
                    match write_result {
                        Ok(r2) => {
                            if r2 > 0 {
                                let image_layer_sha256 = format!("sha256:{}",digest_bytes(&*fs::read(path.clone()).unwrap()));
                                if image_layer_sha256 == layer_digest {
                                    println!("Download image layer successfully！");
                                    let output1 = Command::new("gzip").arg("-d").arg(path.clone()).output();
                                    match output1 {
                                        Ok(res) => {
                                            match String::from_utf8(res.stdout){
                                                Ok(_) => {
                                                     println!("Execute gzip successful!");
                                                    // let image_layer_tar_sha256 = format!("sha256:{}",digest_bytes(&*fs::read(path1.clone()).unwrap()));
                                                    // TODO 获取diffID与image_layer_tar_sha256比较
                                                },
                                                Err(e) => {
                                                     println!("Execute gzip exception! Reason：{}",e);
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            println!("Execute gzip exception! Reason：{}",e);
                                        }
                                    }
                                }else {
                                    println!("Failed to download image layer! The reason is: sha256 is not consistency！")
                                }
                            }else {
                                println!("The image layer is empty！")
                            }
                        }
                        Err(e) => {
                            println!("Failed to download image layer! reason：{}",e)
                        }
                    }
                }
                _ => {}
            }
        },
        Err(e) => {
            println!("Failed to download image layer! Reason：{}",e)
        }
    }
}