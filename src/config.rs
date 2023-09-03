use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub admin_id: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            admin_id: env::var("ADMIN_ID").expect("ADMIN_ID must be set"),
        }
    }
}
