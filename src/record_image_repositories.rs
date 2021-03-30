use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use crate::sled_json::{ TreeWrapper,JSONEncoder };

pub fn record_image_repositories(db: &sled::Db,image_name:String,image_version:String) -> sled::Result<()> {
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ImageVersionJSONValue {
        pub image_version: HashMap<String,String>,
    }

    let mut image_version = HashMap::new();

    image_version.insert("nginx:latest".to_string(),"sha256:6084105296a952523c36eea261af38885f41e9d1d0001b4916fa426e45377ffe".to_string());

    let json_value = ImageVersionJSONValue {
        image_version
    };

    let tree = TreeWrapper::<JSONEncoder<ImageVersionJSONValue>, JSONEncoder<ImageVersionJSONValue>>::new(
        db.open_tree("image_repositories")?,
    );
    tree.insert("nginx".to_string(), &json_value)?;
    let value = tree
        .get("nginx".to_string())?
        .expect("Value not found")
        .decode()
        .expect("Decoding failed");
    assert_eq!(value, json_value);
    println!("image_version:{:?}",value);
    // println!("image_version:{:?}",value.image_version["nginx:latest"]);
    Ok(())
}