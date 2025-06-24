use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Conf {
    pub app: AppConf,
    pub db: DBConf,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppConf {
    pub port: String,
    pub host: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DBConf {
    pub pg_db_connection_port: String,
    pub pg_db_host: String,
}

impl Conf {
    pub async fn get_config(app_name: String) -> Result<Conf, reqwest::Error> {
        if cfg!(debug_assertions) {
            match envy::prefixed(app_name).from_env::<Conf>() {
                Ok(config) => Ok(config),
                Err(error) => panic!("cant parse config {}", error),
            }
        } else {
            match fetch_config_from_server().await {
                Ok(conf) => Ok(conf),
                Err(error) => panic!("fall at fetch conf from server {}", error),
            }
        }
    }
}

async fn fetch_config_from_server() -> Result<Conf, Error> {
    let client = reqwest::Client::new();

    let response: Conf = client
        .post("http://localhost:3030/mystruct")
        .json(&Conf::default())
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
