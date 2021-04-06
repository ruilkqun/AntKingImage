use crate::entrypoint::pull_image;
use crate::utils::create_sled_db;
use crate::get_manifest::get_manifest_info;
use crate::get_image_digest_dockerhub::get_digest_info_dockerhub;

#[tokio::main]
pub async fn cri_pull_image(repositories_url_ip:String,username:String,password:String,image_name:String,image_version:String,docker:bool) -> String {
    let db_tmp = create_sled_db().await;
    let db = match db_tmp{
      Some(res) => res,
      None => return "".to_string()
    };
    pull_image(&db, repositories_url_ip.clone(), image_name.clone(), image_version.clone(), username.clone(), password.clone(), docker.clone()).await;
    return if docker {
        let image_digest_1 = get_digest_info_dockerhub(image_name.clone(), image_version.clone()).await.unwrap();
        let image_digest_2 = image_digest_1.images;
        let mut image_digest = "".to_string();

        for (_, v) in image_digest_2.iter().enumerate() {
            if v.architecture == "amd64".to_string() {
                image_digest = v.digest.clone();
                break;
            }
        }
        image_digest
    } else {
        let manifest_info = get_manifest_info(repositories_url_ip.clone(), username.clone(), password.clone(), image_name.clone(), image_version.clone()).await;
        let manifest_info_1 = match manifest_info {
            Ok(res) => {
                res
            },
            Err(e) => {
                println!("Get manifest_ Info failed!Reason:{}", e);
                return "".to_string()
            }
        };

        let image_digest = manifest_info_1.config.digest.clone();
        image_digest
    }
}

