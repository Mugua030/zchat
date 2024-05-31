use anyhow::Result;
use serde::Deserialize;
use std::io::Read;
use std::{path::Path, fs::File};

#[derive(Debug, Deserialize)]
struct AppCfg {
    server_id: String,
    name: String,
    port: u32,
}

#[derive(Debug, Deserialize)]
struct Database {}

#[derive(Debug, Deserialize)]
struct Log {}

#[derive(Debug, Deserialize)]
struct Config {
    app: AppCfg,
    database: Database,
    log: Log,
}

impl Config {
    fn new(cf: &str) -> Self {
        let acfg = load_from_file(cf);
        let Ok(config) = acfg else {
            panic!("init config fail");
        };

        config
    }
}


// from the config.toml
fn load_from_file(file_path: &str) -> Result<Config> {

    let mut file = File::open(Path::new(file_path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use super::load_from_file;

    #[test]
    fn it_work() {
        let file_path = "/home/martin/workspace/rust/zchat/config/config.toml";
        let cfg = load_from_file(file_path).unwrap();
        println!("{:?}", cfg);
    }
}
