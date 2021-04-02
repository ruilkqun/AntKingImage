use std::process::{Command, Output};
use std::fs;
use tokio::io::Error;


pub async fn compute_layer_size(path:String) -> String{
    let content = fs::read(path.clone()).unwrap();
    let size = content.len();
    return format!("{}",size)
}


pub async fn computer_layer_chain_id(layer_parent_chain_id:String,layer_diff_id:String) -> String {
    if layer_parent_chain_id != "".to_string() {
        let sha256_params = format!("echo -n \"sha256:{} sha256:{}\" | sha256sum",layer_parent_chain_id,layer_diff_id);
        let output1 = Command::new("sh").arg("-c").arg(sha256_params.clone()).output();
            match output1 {
                Ok(res) => {
                    match String::from_utf8(res.stdout){
                        Ok(res1) => {
                            let res1_1 = res1.split(' ');
                            let res1_2: Vec<&str> = res1_1.collect();
                            return format!("{}",res1_2[0])
                        },
                        Err(e) => {
                             println!("Execute sha256sum exception! Reason：{}",e);
                            "".to_string()
                        }
                    }
                }
                Err(e) => {
                    println!("Execute sha256sum exception! Reason：{}",e);
                    "".to_string()
                }
            }
        }else {
           layer_diff_id
        }
}