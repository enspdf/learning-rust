use dotenv::dotenv;
use std::env;

pub struct Config {
    pub port: u16,
    pub host: String,
}

pub fn read_config() -> Config {
    dotenv().ok();

    Config {
        port: env::var("PORT")
            .expect("no port defined")
            .parse()
            .expect("bad port definition"),
        host: env::var("HOST").expect("no host defined"),
    }
}
