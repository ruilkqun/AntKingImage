use crate::entrypoint::pull_image;
use crate::utils::create_sled_db;

// #[tokio::main]
pub async fn cri_pull_image(repositories_url_ip:String,username:String,password:String,image_name:String,image_version:String,docker:bool)  {
    let db_tmp = create_sled_db().await;
    let db = match db_tmp{
      Some(res) => res,
      None => return
    };

    pull_image(&db, repositories_url_ip.clone(), image_name.clone(), image_version.clone(), username.clone(), password.clone(), docker.clone()).await;
}

