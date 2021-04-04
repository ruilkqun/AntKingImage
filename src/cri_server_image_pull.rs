use sled::{ Config, Mode };
use crate::entrypoint::pull_image;


#[tokio::main]
pub async fn cri_pull_image(repositories_url_ip:String,username:String,password:String,image_name:String,image_version:String,docker:bool) -> sled::Result<()> {
    let db = Config::new()
        .mode(Mode::HighThroughput)
        .path("/var/lib/AntKing/db")
        .open()?;

    pull_image(&db, repositories_url_ip, image_name, image_version, username, password, docker).await;

    Ok(())
}
