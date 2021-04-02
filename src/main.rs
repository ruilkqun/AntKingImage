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


use record_image_repositories::record_image_repositories;
use get_manifest::get_manifest_info;
use local_repositories::judge_image_local;
use get_config::{ get_config_info,write_config_json,read_config_json };
use get_layers::get_layers_all;
use record_image_digest_layerdiffid_layerdigest::{ record_image_digest_layer_diff_id_to_layer_digest,record_image_digest_layer_digest_layer_diff_id};
use record_image_layer_level::record_image_layer_diff_id_to_level;
use record_image_chainid::record_image_chain_id;
use utils::{ compute_layer_size,computer_layer_chain_id };
use sled::{ Config, Mode };


#[tokio::main]
pub async fn main() -> sled::Result<()> {
    let db = Config::new()
    .mode(Mode::HighThroughput)
    .path("./ImageDB")
    .open()?;

    println!("computer_layer_chain_id:{}",computer_layer_chain_id("cc967c529ced563b7746b663d98248bc571afdb3c012019d7f54d6c092793b8b".to_string(),"2c6ac8e5063e35e91ab79dfb7330c6154b82f3a7e4724fb1b4475c0a95dfdd33".to_string()).await);
    println!("computer_layer_chain_id:{}",computer_layer_chain_id("".to_string(),"2c6ac8e5063e35e91ab79dfb7330c6154b82f3a7e4724fb1b4475c0a95dfdd33".to_string()).await);


    let image_name = "nginx1".to_string();
    let image_version = "latest".to_string();
    let image_digest = "sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe".to_string();
    record_image_repositories(&db,image_name.clone(),image_version.clone(),image_digest.clone()).await;

    match get_manifest_info("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),"latest".to_string()).await {
        Ok(res) => {
            println!("config digest:{}",res.config.digest);
            println!("layers length:{}",res.layers.len());
            let judge_result_local = judge_image_local(&db,image_name.clone(),image_version.clone(),res.config.digest).await;
            println!("judge_result_local:{}",judge_result_local)
        },
        _ => {}
    }

    // match get_config_info("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone()).await {
    //     Ok(res) => {
    //         println!("layers diff_ids:{}",res.rootfs.diff_ids[0]);
    //     },
    //     Err(e) => {println!("error is:{}",e)}
    // }
    write_config_json("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone()).await;
    get_layers_all("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_version.clone()).await;
    // read_config_json("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone()).await;
    // get_layers("http://192.168.1.118:8899".to_string(),"saodiseng/nginx".to_string(),image_digest.clone(),"sha256:6f28985ad1843afd6fd4fe0b42a30bfab63c27d302362e7341e3316e8ba25ced".to_string()).await;
    record_image_digest_layer_diff_id_to_layer_digest(&db,image_digest.clone(),"2230366c7c6c06d21500c020c2499d9d1b9c0325f281b26adecbe8ae577c94f9".to_string(),"29f7ebf60efda2064ed8f3ca5f748b757c9eb4194e8db766ee370067d2c72210".to_string()).await;
    record_image_digest_layer_digest_layer_diff_id(&db,image_digest.clone(),"2230366c7c6c06d21500c020c2499d9d1b9c0325f281b26adecbe8ae577c94f9".to_string(),"29f7ebf60efda2064ed8f3ca5f748b757c9eb4194e8db766ee370067d2c72210".to_string()).await;
    record_image_layer_diff_id_to_level(&db,image_digest.clone(),"2230366c7c6c06d21500c020c2499d9d1b9c0325f281b26adecbe8ae577c94f9".to_string(),0).await;
    record_image_chain_id(&db,image_digest.clone(),"14a1ca976738392ffa2ae4e54934ba28ab9cb756e924ad9297a4795a4adbfdf6".to_string(),"12ae384727752b6926bee0103a718c921524b7bfb11804eb8b4babd5b012335e".to_string(),"sha256:14a1ca976738392ffa2ae4e54934ba28ab9cb756e924ad9297a4795a4adbfdf6".to_string(),"".to_string(),"69221313".to_string()).await;
    println!("compute_layer_size:{}",compute_layer_size("./6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe/158aac73782cb5bf2f03cc3b3f9afa49ce582c26a546f6dba65994d1c7ddd43d.tar".to_string()).await);
    Ok(())
}
