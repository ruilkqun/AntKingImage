use crate::entrypoint::remove_image;
use crate::utils::create_sled_db;


// #[tokio::main]
pub async fn cri_remove_image(image_digest:String)  {
    let db_tmp = create_sled_db().await;
    let db = match db_tmp{
      Some(res) => res,
      None => return
    };

    remove_image(&db, image_digest.clone()).await;
}