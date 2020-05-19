use rusoto_core::Region;
use rusoto_secretsmanager::{
    GetSecretValueRequest, GetSecretValueResponse, SecretsManager, SecretsManagerClient,
};
use serde_json::Value;
use tokio::runtime::Runtime;

pub fn get_secret(secret_name: String) -> String {
    let req = GetSecretValueRequest {
        secret_id: String::from("dev/four_digits"),
        version_id: None,
        version_stage: None,
    };
    let client = SecretsManagerClient::new(Region::UsEast1);
    let resp: GetSecretValueResponse = match Runtime::new()
        .unwrap()
        .block_on(SecretsManager::get_secret_value(&client, req))
    {
        Ok(res) => res,
        Err(e) => {println!("error:{}",e);panic!(e)},
    };

    let secret_string: String = match resp.secret_string {
        Some(s) => s,
        None => String::from(""),
    };

    let secret_pairs: Value = match serde_json::from_str(&secret_string) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };

    format!("{}", secret_pairs[secret_name].as_str().unwrap())
}
