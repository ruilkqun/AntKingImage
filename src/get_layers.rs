use reqwest::header::HeaderMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use sha256::digest_bytes;
use std::process::Command;
// use crate::get_manifest::get_manifest_info;

// pub async fn get_layers_all(repositories_url_ip:String,image_name:String,image_version:String){
//         match get_manifest_info(repositories_url_ip.clone(),image_name.clone(),image_version.clone()).await {
//         Ok(res) => {
//             for i in 0..res.layers.len() {
//                 get_layers(repositories_url_ip.clone(),image_name.clone(),res.config.digest.clone(),res.layers[i].digest.clone()).await;
//             }
//         },
//         _ => {}
//     }
// }


pub async fn get_layers(repositories_url_ip:String,username:String,password:String,image_name:String,image_digest:String,layer_digest:String,layer_diff_id:String)  {
    let url = format!("{}/v2/{}/blobs/{}", repositories_url_ip, image_name, layer_digest);
    // println!("get_layers_url:{}",url);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.image.rootfs.diff.tar.gzip".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());

    let image_storage_path_1 = image_digest.split(':');
    let image_storage_path_2: Vec<&str> = image_storage_path_1.collect();
    let image_storage_path_3 = image_storage_path_2[1];
    let image_storage_path_4 = format!("{}",image_storage_path_3.clone());

    let image_storage_path = format!("/var/lib/AntKing/images/{}",image_storage_path_4);
    fs::create_dir_all(image_storage_path.clone()).unwrap();

    let layer_name_tmp = layer_digest.split(':');
    let layer_name: Vec<&str> = layer_name_tmp.collect();


    let path = format!("{}/{}.tar.gz",image_storage_path.clone(),layer_name[1]);
    let path1 = format!("{}/{}.tar",image_storage_path.clone(),layer_name[1]);


    let layer_storage_path_1 = layer_diff_id.split(':');
    let layer_storage_path_2: Vec<&str> = layer_storage_path_1.collect();
    let layer_storage_path_3 = layer_storage_path_2[1];
    let layer_storage_path_4 = format!("{}",layer_storage_path_3.clone());
    let path2 = format!("{}/{}",image_storage_path.clone(),layer_storage_path_4.clone());

    fs::create_dir_all(path2.clone()).unwrap();


    match fs::remove_file(path1.clone()) {
        Ok(()) => {
            println!("Delete previous image layer file successfully！")
        },
        Err(e) => {
            println!("Failed to delete previous image layer file! Reason：{}",e)
        }
    }
    let mut file = File::create(path.clone()).unwrap();



    match client.get(url).basic_auth(username,Some(password)).headers(headers).send().await {
        Ok(r) => {
            match r.bytes().await {
                Ok(r1) => {
                    let write_result = file.write(&*r1);
                    match write_result {
                        Ok(r2) => {
                            if r2 > 0 {
                                let image_layer_sha256 = format!("sha256:{}",digest_bytes(&*fs::read(path.clone()).unwrap()));
                                //
                                // println!("image_layer_sha256:{}",image_layer_sha256);
                                // println!("layer_digest:{}",layer_digest);

                                if image_layer_sha256 == layer_digest {
                                    println!("Download image layer successfully！");
                                    let output1 = Command::new("gzip").arg("-d").arg(path.clone()).output();
                                    match output1 {
                                        Ok(res) => {
                                            match String::from_utf8(res.stdout){
                                                Ok(_) => {
                                                     println!("Execute gzip successful!");
                                                    let image_layer_tar_sha256 = format!("sha256:{}",digest_bytes(&*fs::read(path1.clone()).unwrap()));
                                                    // TODO 获取diffID与image_layer_tar_sha256比较
                                                    if image_layer_tar_sha256 == layer_diff_id.clone() {
                                                        println!("Image integrity");
                                                        let cmd = format!("tar -xvf {} -C {}",path1,path2);
                                                        let output2 = Command::new("sh").arg("-c").arg(cmd.clone()).output();
                                                        match output2 {
                                                            Ok(_) => {
                                                                println!("Execute tar successful!");
                                                            },
                                                            Err(e) => {
                                                                println!("Execute tar exception! Reason：{}",e);
                                                            }
                                                        }
                                                    }
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