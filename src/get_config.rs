use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use serde_json::Value;


// 写入当前镜像的配置json文件
pub async fn write_config_json(repositories_url_ip:String,image_name:String,image_digest:String)  {
    let url = format!("{}/v2/{}/blobs/{}",repositories_url_ip.clone(),image_name.clone(),image_digest.clone());
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.container.image.v1+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());


    let path = format!("./{}.json",image_digest.clone());
    match fs::remove_file(path.clone()) {
        Ok(()) => {
            println!("删除 以前 镜像配置文件 成功！")
        },
        Err(e) => {
            println!("删除 以前 镜像配置文件 失败! 原因：{}",e)
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
                                println!("下载镜像配置文件 成功！")
                            }else {
                                println!("镜像配置文件 为空！")
                            }
                        }
                        Err(e) => {
                            println!("下载镜像配置文件 失败！原因：{}",e)
                        }
                    }
                }
                _ => {}
            }
        },
        Err(e) => {
            println!("下载镜像配置文件 失败！原因：{}",e)
        }
    }
}

// 读取当前镜像的配置json文件
pub async fn read_config_json(image_digest:String)  {
    let path = format!("./{}.json",image_digest.clone());
    let read_result = fs::read_to_string(path.clone());
    match read_result {
        Ok(res) => {
            let v = serde_json::from_str(&*res);
            match v {
                Ok(res1) => {
                    let v1:Value = res1;
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