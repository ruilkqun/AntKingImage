use reqwest::header::HeaderMap;
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use std::io::prelude::*;


pub async fn get_layers(repositories_url_ip:String,image_name:String,layer_digest:String)  {
    let url = format!("{}/v2/{}/blobs/{}", repositories_url_ip, image_name, layer_digest);
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "application/vnd.docker.image.rootfs.diff.tar.gzip".parse().unwrap());
    headers.insert("Accept-Language", "zh-CN,zh;q=0.9,zh-TW;q=0.8,en-US;q=0.7,en;q=0.6".parse().unwrap());

    let path = format!("./{}",layer_digest);
    match fs::remove_file(path.clone()) {
        Ok(()) => {
            println!("删除 以前 镜像层 文件 成功！")
        },
        Err(e) => {
            println!("删除 以前 镜像层 文件 失败! 原因：{}",e)
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
                                println!("下载镜像层 成功！")
                            }else {
                                println!("镜像层为空！")
                            }
                        }
                        Err(e) => {
                            println!("下载镜像层 失败！原因：{}",e)
                        }
                    }
                }
                _ => {}
            }
        },
        Err(e) => {
            println!("下载镜像层 失败！原因：{}",e)
        }
    }
}