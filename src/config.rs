use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub api_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            api_url: env::var("API_URL").expect("API_URL must be set"),
        }
    }
}
