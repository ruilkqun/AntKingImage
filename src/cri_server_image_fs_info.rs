use crate::utils::computer_fs_info;
use chrono::Local;

pub async fn cri_image_fs() -> (i64,String,u64,u64) {
    let timestamp = Local::now().timestamp_nanos();
    let mountpoint = "/".to_string();
    let used_bytes:u64;
    let inodes_used:u64;

    let computer_result = computer_fs_info();
    used_bytes = computer_result.0;
    inodes_used = computer_result.1;

    return (timestamp,mountpoint,used_bytes,inodes_used)
}