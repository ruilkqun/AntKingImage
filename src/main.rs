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

use sled::{ Config, Mode };
use entrypoint::pull_image;


#[tokio::main]
pub async fn main() -> sled::Result<()> {
    let db = Config::new()
    .mode(Mode::HighThroughput)
    .path("/var/lib/AntKing/imagedb")
    .open()?;

    let repositories_url_ip = "http://192.168.1.118:8899".to_string();
    let image_name = "saodiseng/nginx".to_string();
    let image_version = "latest".to_string();
    let username = "admin".to_string();
    let password = "saodiseng".to_string();
    pull_image(&db,repositories_url_ip,image_name,image_version,username,password).await;

    Ok(())
}
