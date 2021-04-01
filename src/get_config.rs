use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde_json::Value;
use sha256::digest;
use crate::get_layers::get_layers;


// 写入当前镜像的配置json文件
pub async fn write_config_json(repositories_url_ip:String,image_name:String,image_digest:String)  {
    let url = format!("{}/v2/{}/blobs/{}",repositories_url_ip.clone(),image_name.clone(),image_digest.clone());
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.container.image.v1+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());



    let image_storage_path_1 = image_digest.split(':');
    let image_storage_path_2: Vec<&str> = image_storage_path_1.collect();
    let image_storage_path_3 = image_storage_path_2[1];
    let image_storage_path = format!("{}",image_storage_path_3.clone());
    fs::create_dir_all(image_storage_path.clone()).unwrap();
    let path = format!("{}/{}.json",image_storage_path_3.clone(),image_storage_path_3.clone());
    // println!("config.json path:{}",path);
    match fs::remove_file(path.clone()) {
        Ok(()) => {
            println!("Delete previous image profile successfully！")
        },
        Err(e) => {
            println!("Failed to delete previous image configuration file! Reason：{}",e)
        }
    }
    let mut file = File::create(path.clone()).unwrap();

    match client.get(url).basic_auth("admin",Some("saodiseng")).headers(headers).timeout(std::time::Duration::from_secs(5)).send().await {
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
pub async fn read_config_json(repositories_url_ip:String,image_name:String,image_digest:String)  {
    let image_storage_path_1 = image_digest.split(':');
    let image_storage_path_2: Vec<&str> = image_storage_path_1.collect();
    let image_storage_path_3 = image_storage_path_2[1];
    let path = format!("{}/{}.json",image_storage_path_3.clone(),image_storage_path_3.clone());
    let read_result = fs::read_to_string(path.clone());
    match read_result {
        Ok(res) => {
            let v = serde_json::from_str(&*res);
            match v {
                Ok(res1) => {
                    let v1:Value = res1;
                    // let layer_1 = format!("{}",v1["rootfs"]["diff_ids"]);
                    // let layer_2 = layer_1.split(',');
                    // let layer_3: Vec<&str> = layer_2.collect();
                    //
                    // for i in 0..layer_3.len(){
                    //     let layer_digest_1 = format!("{}",v1["rootfs"]["diff_ids"][i]);
                    //     let layer_digest_2 = layer_digest_1.split('"');
                    //     let layer_digest_3: Vec<&str> = layer_digest_2.collect();
                    //     let layer_digest = format!("{}",layer_digest_3[1]);
                    //     get_layers(repositories_url_ip.clone(),image_name.clone(),image_digest.clone(),layer_digest.clone()).await;
                    // }
                    println!("layers diff_ids:{}",v1["rootfs"]["diff_ids"][0])
                }
                _ => {}
            }
        },

        _ => {}
    }
}





// 最初使用该方式，已弃用(仅留作记录)
// 从远程仓库接口获取当前镜像的配置信息(需要反序列化)
#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Manifest {
    pub architecture: String,
    pub config: Config,
    pub container: String,
    pub container_config: ContainerConfig,
    pub created: String,
    pub docker_version: String,
    // pub history: Vec<HashMap<String,String>>,
    pub os: String,
    pub rootfs: Rootfs,
}


#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Config {
    pub AttachStderr: bool,
    pub AttachStdin: bool,
    pub AttachStdout: bool,
    pub Cmd: Vec<String>,
    pub Domainname: String,
    pub Entrypoint: Vec<String>,
    pub Env: Vec<String>,
    pub ExposedPorts: HashMap<String,HashMap<String,String>>,
    pub Hostname: String,
    pub Image: String,
    pub Labels: HashMap<String,String>,
    pub OnBuild: Option<Vec<String>>,
    pub OpenStdin: bool,
    pub StdinOnce: bool,
    pub StopSignal: String,
    pub Tty: bool,
    pub User: String,
    pub Volumes: Option<HashMap<String,HashMap<String,String>>>,
    pub WorkingDir: String,
}



#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct ContainerConfig {
    pub AttachStderr: bool,
    pub AttachStdin: bool,
    pub AttachStdout: bool,
    pub Cmd: Vec<String>,
    pub Domainname: String,
    pub Entrypoint: Vec<String>,
    pub Env: Vec<String>,
    pub ExposedPorts: HashMap<String,HashMap<String,String>>,
    pub Hostname: String,
    pub Image: String,
    pub Labels: HashMap<String,String>,
    pub OnBuild: Option<Vec<String>>,
    pub OpenStdin: bool,
    pub StdinOnce: bool,
    pub StopSignal: String,
    pub Tty: bool,
    pub User: String,
    pub Volumes: Option<HashMap<String,HashMap<String,String>>>,
    pub WorkingDir: String,
}




#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Rootfs {
    pub diff_ids: Vec<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type1: String,
}



pub async fn get_config_info(repositories_url_ip:String,image_name:String,image_digest:String) -> Result<Manifest, Box<dyn Error>> {
    let url = format!("{}/v2/{}/blobs/{}",repositories_url_ip,image_name,image_digest);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.container.image.v1+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());


    match client.get(url).basic_auth("admin",Some("saodiseng")).headers(headers).timeout(std::time::Duration::from_secs(5)).send().await {
        Ok(r) => {
            match r.json().await {
                Ok(res) => Ok(res),
                Err(e) => {
                    Err(Box::new(e))
                }
            }
        },
        Err(e) => {
            Err(Box::new(e))
        }
    }
}