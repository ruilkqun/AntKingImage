/// use AntKingImage::cri_server_image_pull::cri_pull_image
///
/// docker hub 镜像
/// cri_pull_image("".to_string,"".to_string,"".to_string,image_name,image_version,docker:true).await.unwrap;
///
/// 自建仓库 镜像
/// cri_pull_image(repositories_url_ip,username,password,"".to_string,"".to_string,"".to_string,image_name,image_version,docker:true).await.unwrap;
///
/// Examples:
/// Download image configuration file successfully！
/// [00:00:04] ======================================== 27092654/27092654 000eee12ec04cc914bf96e8f5dee7767510c2aca3816af6078bd9fbe3150920c downloaded
/// [00:00:06] ======================================== 23741065/23741065 eb22865337de3edb54ec8b52f6c06de320f415e7ec43f01426fdafb8df6d6eb7 downloaded
/// [00:00:02] ========================================     203/203     bee5d581ef8bfee2b5a54685813ba6ad9bbe922115d7aef84a21a9dbfcc2d979 downloaded
/// [00:00:02] ========================================     549/549     a8ad52858f44c72f28dd1649338e4ec2dc9119992eb100876b05a6f783ab2667 downloaded
/// Download Image ruilkyu/nginx:latest complete!


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
