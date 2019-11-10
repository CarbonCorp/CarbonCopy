extern crate serde_json;
extern crate shiplift;
extern crate tokio;
extern crate yaml_rust;
use tokio::prelude::{Future, Stream};
mod config;

fn main() {
    // read Carbonfile
    let config = config::get_config();
    let docker = shiplift::Docker::new();

    start_docker(&config, &docker);
}

fn start_docker(config: &yaml_rust::Yaml, docker: &shiplift::Docker) {
    let img = shiplift::PullOptions::builder()
        .image(config["environment"]["image"].as_str().unwrap())
        .build();
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
}
