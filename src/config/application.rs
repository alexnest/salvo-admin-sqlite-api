use crate::util::from::toml::Toml;
use crate::util::from::From;
// use crate::util::from::yml::Yml; // if you use the .yml or . yaml file, you need to uncomment the following code and comment the above code
use serde::Deserialize;
// use once_cell::sync::Lazy;

#[derive(Deserialize)]
pub struct Server {
    pub host: String,
    pub req_max_size: u32,
}

#[derive(Deserialize)]
pub struct Log {
    // log output level
    pub log_level: String,
    // log output folder
    pub dir: String,
    // log output file name
    pub file: String,
}

#[derive(Deserialize)]
pub struct Database {
    pub host: String,
    pub kv_host: String,
}

#[derive(Deserialize)]
pub struct Jwt {
    pub exp: u64,
    pub secrect_key: String,
}

#[derive(Deserialize)]
pub struct Application {
    pub server: Server,
    pub log: Log,
    pub database: Database,
    pub jwt: Jwt,
}

// use the toml file as the config file
pub const APPLICATION_PATH: &str = "env/application.toml";

// use the yml file as the config file(if you use the .yml or . yaml file, you need to uncomment the following code and comment the above code)
// pub const APPLICATION_PATH: &str = "env/application.yml";

impl Application {
    pub fn init() -> Application {
        // transform the toml file into the target object
        let config: Application = Toml.from_path(APPLICATION_PATH);

        // transform the yml file into the target object (if you use the .yml or . yaml file, you need to uncomment the following code and comment the above code)
        // let config: Application = Yml.from_path(APPLICATION_PATH);
        config.validate();
        config
    }

    fn validate(&self) {
        if self.server.host.is_empty() {
            panic!("please make correct host in config file")
        }
    }
}
