use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest::header::HeaderMap;


#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct Digest {
    pub images: Vec<DigestItem>,
}

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct DigestItem {
    pub architecture: String,
    pub digest:String,
}




pub async fn get_digest_info_dockerhub(image_name:String,image_version:String) -> Result<Digest, Box<dyn Error>> {
    let url = format!("https://registry.hub.docker.com/v2/repositories/{}/tags/{}",image_name.clone(),image_version.clone());
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());


    match client.get(url).timeout(std::time::Duration::from_secs(10)).send().await {
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