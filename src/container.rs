use std::env;
use tokio::prelude::{Future, Stream};

pub fn start_container(config: &yaml_rust::Yaml, docker: &shiplift::Docker) {
    // pull the image
    let image_name = config["environment"]["image"].as_str().unwrap();

    let container_name = get_image_name();
    let img = shiplift::PullOptions::builder().image(image_name).build();
    let fut = docker
        .images()
        .pull(&img)
        .for_each(|output| {
            if output["id"] != serde_json::Value::Null {
                print!("{}", output["id"].as_str().unwrap());
            }
            if output["status"] != serde_json::Value::Null {
                print!(" {}", output["status"].as_str().unwrap());
            }
            if output["progress"] != serde_json::Value::Null {
                print!(" {}", output["progress"].as_str().unwrap());
            }
            println!("");
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(fut);

    // check if container already exist, else create
    let container = docker.containers().get(&container_name);
    let del_fut = container.delete().map_err(|e| eprintln!("Error: {}", e));
    tokio::run(del_fut);
    println!("removed existing container {:?}", container.id());

    // create the container
    let fut = docker
        .containers()
        .create(
            &shiplift::ContainerOptions::builder(image_name.as_ref())
                .name(&container_name)
                .build(),
        )
        .map(|info| println!("{:?}", info))
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(fut);
}

fn get_image_name() -> String {
    // generates the image name
    let path = env::current_dir();
    let path = match path {
        Ok(dir) => dir,
        Err(e) => panic!("Problem accessing the current directory : {:?}", e),
    };
    let dir = path.file_name();
    let dir = match dir {
        Some(d) => match d.to_str() {
            Some(t) => t.to_string(),
            None => String::from(""),
        },
        None => String::from(""),
    };
    let mut container_name = String::from("carboncopy_");
    container_name.push_str(&dir);
    container_name
}
