pub mod sled_json;
pub mod record_image_repositories;
pub mod public_struct;
pub mod get_manifest;
pub mod local_repositories;
pub mod get_config;
pub mod get_layers;


use record_image_repositories::record_image_repositories;
use get_manifest::get_manifest_info;
use local_repositories::judge_image_local;
use get_config::{ get_config_info,write_config_json,read_config_json };
use get_layers::get_layers;
use sled::{ Config, Mode };


#[tokio::main]
pub async fn main() -> sled::Result<()> {
    let db = Config::new()
    .mode(Mode::HighThroughput)
    .path("./ImageDB")
    .open()?;

    let image_name = "nginx".to_string();
    let image_version = "latest".to_string();
    let image_digest = "sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe".to_string();
    record_image_repositories(&db,image_name.clone(),image_version.clone(),image_digest.clone())?;

    match get_manifest_info("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),"latest".to_string()).await {
        Ok(res) => {
            println!("config digest:{}",res.config.digest);
            println!("layers length:{}",res.layers.len());
            let judge_result_local = judge_image_local(&db,image_name.clone(),image_version.clone(),res.config.digest).await;
            println!("judge_result_local:{}",judge_result_local)
        },
        _ => {}
    }

    match get_config_info("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone()).await {
        Ok(res) => {
            println!("layers diff_ids:{}",res.rootfs.diff_ids[0]);
        },
        Err(e) => {println!("error is:{}",e)}
    }
    write_config_json("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone()).await;
    read_config_json(image_digest.clone()).await;
    get_layers("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),"sha256:6f28985ad1843afd6fd4fe0b42a30bfab63c27d302362e7341e3316e8ba25ced".to_string()).await;

    Ok(())
}
