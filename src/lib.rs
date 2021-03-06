//! AntKingImage
//!
//! A Library For Download OCI Image
//!
//! Example:
//!
//! 1、Downloade DockerHub Image
//!
//! cri_pull_image("".to_string,"".to_string,"".to_string,image_name,image_version,docker:true).await.unwrap;
//!
//! 2、Download Registry Image
//!
//! cri_pull_image(repositories_url_ip,username,password,image_name,image_version,docker:true).await.unwrap;

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
pub mod get_image_list;
pub mod get_image_size;
pub mod record_image_digest_image_name_version_repositories;
pub mod cri_server_image_remove;
pub mod get_image_status;
pub mod cri_server_image_fs_info;


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

// #[tokio::main]
// pub async fn main() {
    // let reply_tmp = get_image_status::read_image_status_repositories("519e12e2a84a9".to_string()).await;
    // println!("{:?}",reply_tmp);
    //
    // let reply_tmp1 = get_image_list::read_image_list_repositories("".to_string(),"".to_string(),"sha256:519e12e2a84a9eb18094635ae1edfd97b26f95dbc66e317eefb657a1cb08c8dc".to_string()).await;
    // println!("{:?}",reply_tmp1)

//     let reply_tmp2 = utils::computer_fs_info();
//     println!("{:?}",reply_tmp2);
// }