pub mod sled_json;
pub mod record_image_repositories;
pub mod public_struct;
pub mod get_manifest;
pub mod local_repositories;
pub mod get_config;
pub mod get_layers;
pub mod record_image_digest_layerdiffid_layerdigest;
pub mod record_image_chainid;
pub mod record_image_layer_level;
pub mod utils;
pub mod entrypoint;
pub mod get_image_digest_dockerhub;
pub mod get_manifest_dockerhub;
pub mod get_token_dockerhub;
pub mod get_config_dockerhub;
pub mod get_layers_dockerhub;
pub mod cri_server_image_pull;



// use sled::{ Config, Mode };
// use entrypoint::pull_image;
// #[tokio::main]
// pub async fn main() -> sled::Result<()> {
//     let db = Config::new()
//     .mode(Mode::HighThroughput)
//     .path("/var/lib/AntKing/imagedb")
//     .open()?;
//
//     let repositories_url_ip = "http://192.168.1.118:8899".to_string();
//     // let image_name = "saodiseng/nginx".to_string();
//     // let image_version = "hi".to_string();
//     let image_name = "ruilkyu/nginx".to_string();
//     let image_version = "latest".to_string();
//     let username = "".to_string();
//     let password = "".to_string();
//     let docker = true;
//     pull_image(&db,repositories_url_ip,image_name,image_version,username,password,docker).await;
//
//     Ok(())
// }
