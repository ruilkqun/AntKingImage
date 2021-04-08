## ant-king-image (OCI镜像拉取，存储)
```
A Library For Download OCI Image

Example:

1、Downloade DockerHub Image

   cri_pull_image("".to_string,"".to_string,"".to_string,image_name,image_version,docker:true).await.unwrap;

2、Download Registry Image

   cri_pull_image(repositories_url_ip,username,password,image_name,image_version,docker:false).await.unwrap;

3、运行

Download image configuration file successfully！
[00:00:04] ======================================== 27092654/27092654 000eee12ec04cc914bf96e8f5dee7767510c2aca3816af6078bd9fbe3150920c downloaded
[00:00:06] ======================================== 23741065/23741065 eb22865337de3edb54ec8b52f6c06de320f415e7ec43f01426fdafb8df6d6eb7 downloaded
[00:00:02] ========================================     203/203     bee5d581ef8bfee2b5a54685813ba6ad9bbe922115d7aef84a21a9dbfcc2d979 downloaded
[00:00:02] ========================================     549/549     a8ad52858f44c72f28dd1649338e4ec2dc9119992eb100876b05a6f783ab2667 downloaded
Download Image ruilkyu/nginx:latest complete!
```

##  与Kubernetes CRI对接
### 1、源码实例
```
use ant_king_image::cri_server_image_pull::cri_pull_image;
use ant_king_image::local_repositories::get_image_digest_local;

pub async fn pull_image_impl_v1alpha2(request:Request<PullImageRequest>) -> PullImageResponse {
        // docker:nginx:latest
        // 192.168.1.118:8899/saodiseng/nginx:latest
        let pull_image_request = request.into_inner();
        let image_tmp1 = pull_image_request.clone().image;
        let auth = pull_image_request.clone().auth;
        let _sandbox_config = pull_image_request.clone().sandbox_config;

        let image_tmp2 = match image_tmp1 {
                Some(res) => res,
                None => {
                        let reply = PullImageResponse {
                                image_ref: "".to_string()
                        };
                        return reply
                }
        };

        let image_analysis1 = image_tmp2.image.split(":");
        let image_analysis2:Vec<&str> = image_analysis1.collect();
        let docker = image_analysis2[0];

        return if docker == "docker" {
                let image_name = image_analysis2[1];
                let image_version = image_analysis2[2];

                cri_pull_image("".to_string(), "".to_string(), "".to_string(), image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap(), true).await;
                let image_digest_1 = get_image_digest_local(image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}@{}",image_name.clone(),image_digest_1.clone());
                println!("image_digest:{}",image_digest.clone());
                let reply = PullImageResponse {
                        image_ref: image_digest.clone()
                };
                reply
        } else {
                let image_auth = match auth {
                        Some(res) => res,
                        None => {
                                let reply = PullImageResponse {
                                        image_ref: "".to_string()
                                };
                                return reply
                        }
                };
                let username = image_auth.username;
                let password = image_auth.password;
                let image_version = image_analysis2[2];
                let tmp1 = image_analysis2[1].split("/");
                let tmp2:Vec<&str> = tmp1.collect();
                let tmp3 = format!("{}:{}",image_analysis2[0],tmp2[0]);
                let mut tmp4 = "".to_string();
                for l in 1..tmp2.len() {
                        tmp4 += &*("/".to_string() + tmp2[l]);
                }
                let image_name = format!("{}",tmp4);

                cri_pull_image(tmp3, username, password, image_name.parse().unwrap(), image_version.parse().unwrap(), true).await;
                let image_digest_1 = get_image_digest_local(image_name.clone().parse().unwrap(), image_version.clone().parse().unwrap()).await.unwrap();
                let image_digest = format!("{}@{}",image_name.clone(),image_digest_1.clone());
                println!("image_digest:{}",image_digest.clone());

                let reply = PullImageResponse {
                        image_ref: image_digest.clone()
                };
                reply
        }
}
```

### 2、运行实例
```
[root@localhost container]# crictl --image-endpoint unix:///var/run/saodiseng.sock  pull docker:ruilkyu/nginx:latest

Image is up to date for ruilkyu/nginx@sha256:bd877619f4ab21d0d2a26c622c0c51935d4da763203d83f542e39a4720d09bdc
```


