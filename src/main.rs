extern crate serde_json;
extern crate shiplift;
extern crate tokio;
extern crate yaml_rust;
mod config;
mod container;

fn main() {
    // read Carbonfile
    let config = config::get_config();
    let docker = shiplift::Docker::new();

    container::start_container(&config, &docker);
}
