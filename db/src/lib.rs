pub mod config;
pub mod draw_results;

use mongodb::{error::Error, sync::Client};

pub fn connect(conf: config::Config) -> Result<Client, Error> {
    let url = format!(
        "mongodb+srv://{}:{}@{}/{}?retryWrites=true",
        conf.username,
        conf.password,
        conf.host,
        config::DB_NAME
    );
    Client::with_uri_str(&url)
}
