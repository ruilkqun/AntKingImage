pub mod sled_json;
pub mod record_image_repositories;
pub mod public_struct;


use record_image_repositories::record_image_repositories;
use sled::{ Config, Mode };

fn main() -> sled::Result<()> {
    let db = Config::new()
    .mode(Mode::HighThroughput)
    .path("./ImageDB")
    .open()?;

    let image_name = "nginx".to_string();
    let image_version = "latest".to_string();
    let image_digest = "sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe2".to_string();
    record_image_repositories(&db,image_name,image_version,image_digest)?;
    Ok(())
}
