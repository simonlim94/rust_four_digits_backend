use util::secret_manager;

#[derive(Debug)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub host: String,
}

pub static DB_NAME: &str = "four_digits";
pub static DRAW_RESULTS_COLLECTION: &str = "draw_results";

impl Config {
    pub fn new() -> Self {
        Config {
            username: secret_manager::get_secret(String::from("db_username")),
            password: secret_manager::get_secret(String::from("db_password")),
            host: secret_manager::get_secret(String::from("db_host")),
        }
    }
}
