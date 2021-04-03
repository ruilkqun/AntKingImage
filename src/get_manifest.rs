use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::header::HeaderMap;


#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Manifest {
    pub config: Config,
    pub layers: Vec<Layers>,
    pub mediaType: String,
    pub schemaVersion: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Config {
    pub digest: String,
    pub mediaType: String,
    pub size: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Layers {
    pub digest: String,
    pub mediaType: String,
    pub size: i64,
}


pub async fn get_manifest_info(repositories_url_ip:String,username:String,password:String,image_name:String,image_version:String) -> Result<Manifest, Box<dyn Error>> {
    let url = format!("{}/v2/{}/manifests/{}",repositories_url_ip,image_name,image_version);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.distribution.manifest.v2+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());


    match client.get(url).basic_auth(username,Some(password)).headers(headers).timeout(std::time::Duration::from_secs(5)).send().await {
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