pub mod sled_json;
pub mod record_image_repositories;


use record_image_repositories::record_image_repositories;
use sled::{ Config, Mode };

fn main() -> sled::Result<()> {
    let db = Config::new()
    .mode(Mode::HighThroughput)
    .path("./ImageDB")
    .open()?;

    record_image_repositories(&db)?;
    Ok(())
}
