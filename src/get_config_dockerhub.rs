use reqwest::header::HeaderMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde_json::Value;
use sha256::digest;


// 写入当前镜像的配置json文件
pub async fn write_config_json_dockerhub(image_name:String,image_digest:String,token:String)  {
    let url = format!("https://registry.hub.docker.com/v2/{}/blobs/{}",image_name.clone(),image_digest.clone());
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.container.image.v1+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());

    let image_storage_path_1 = image_digest.split(':');
    let image_storage_path_2: Vec<&str> = image_storage_path_1.collect();
    let image_storage_path_3 = image_storage_path_2[1];
    let image_storage_path_4 = format!("{}",image_storage_path_3.clone());

    let image_storage_path = format!("/var/lib/AntKing/images/{}",image_storage_path_4);
    fs::create_dir_all(image_storage_path.clone()).unwrap();

    let path = format!("{}/{}.json",image_storage_path.clone(),image_storage_path_4.clone());
    let mut file = File::create(path.clone()).unwrap();

    match client.get(url).bearer_auth(token.clone()).timeout(std::time::Duration::from_secs(10)).send().await {
        Ok(r) => {
            match r.bytes().await {
                Ok(r1) => {
                    let write_result = file.write(&*r1);
                    match write_result {
                        Ok(r2) => {
                            if r2 > 0 {
                                let config_json_sha256 = format!("sha256:{}",digest(fs::read_to_string(path.clone()).unwrap()));
                                if config_json_sha256 == image_digest {
                                    println!("Download image configuration file successfully！")
                                }else {
                                    println!("Failed to download image configuration file! Reason: sha256 is not match")
                                }
                            }else {
                                println!("The image configuration file is empty！")
                            }
                        }
                        Err(e) => {
                            println!("Failed to download image configuration file! Reason：{}",e)
                        }
                    }
                }
                _ => {}
            }
        },
        Err(e) => {
            println!("Failed to download image configuration file! Reason：{}",e)
        }
    }
}

// 读取当前镜像的配置json文件
pub async fn read_config_json_dockerhub(image_digest:String) -> Value {
    let image_storage_path = format!("/var/lib/AntKing/images/{}",image_digest);

    let path = format!("{}/{}.json",image_storage_path.clone(),image_digest.clone());
    let read_result = fs::read_to_string(path.clone());
    match read_result {
        Ok(res) => {
            let v = serde_json::from_str(&*res);
            match v {
                Ok(res1) => {
                    let v1:Value = res1;
                    v1
                }
                _ => {
                    Value::default()
                }
            }
        },

        _ => {
            Value::default()
        }
    }
}