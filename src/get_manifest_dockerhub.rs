use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::header::HeaderMap;
use crate::get_token_dockerhub::get_token_dockerhub;
use crate::get_image_digest_dockerhub::get_digest_info_dockerhub;


#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Manifest {
    pub schemaVersion: i64,
    pub mediaType: String,
    pub config: Config,
    pub layers: Vec<Layers>,
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


pub async fn get_manifest_info_dockerhub(image_name:String,image_version:String) -> Result<Manifest, Box<dyn Error + Send>> {
    let image_digest_1 = get_digest_info_dockerhub(image_name.clone(),image_version.clone()).await.unwrap();
    let image_digest_2 = image_digest_1.images;
    let mut image_digest = "".to_string();

    for (_,v) in image_digest_2.iter().enumerate(){
        if v.architecture == "amd64".to_string() {
            image_digest = v.digest.clone();
            break;
        }
    }
    // println!("image_digest:{}",image_digest);

    let url = format!("https://registry.hub.docker.com/v2/{}/manifests/{}",image_name.clone(),image_digest.clone());
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/vnd.docker.distribution.manifest.v2+json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());
    let token_1 = get_token_dockerhub(image_name.clone()).await.unwrap();
    let token = format!("{}",token_1.access_token);


    match client.get(url.clone()).bearer_auth(token.clone()).timeout(std::time::Duration::from_secs(10)).send().await {
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