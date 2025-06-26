use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Conf {
    pub pg_db_connection_port: String,
    pub pg_db_host: String,
    pub pg_db_name: String,
    pub pg_db_username: String,
    pub pg_db_password: String,
    pub port: String,
    pub host: String,
}

#[derive(Serialize, Deserialize)]
struct ReqStruct {
    app_name: String,
}

pub fn testetset() {
    dotenv().ok();

    let x: Conf = envy::from_env().unwrap();
    println!("{:?}", x)
}

pub async fn get_config(app_name: String) -> Result<Conf, reqwest::Error> {
    let formated_app_name = format!("{}_", app_name.to_uppercase());
    if cfg!(debug_assertions) {
        dotenv().ok();
        match envy::prefixed(formated_app_name).from_env::<Conf>() {
            Ok(config) => Ok(config),
            Err(error) => panic!("cant parse config {}", error),
        }
    } else {
        match fetch_config_from_server(app_name).await {
            Ok(conf) => Ok(conf),
            Err(error) => panic!("fall at fetch conf from server {}", error),
        }
    }
}

async fn fetch_config_from_server(app_name: String) -> Result<Conf, Error> {
    let client = reqwest::Client::new();
    let response: Conf = client
        .post("http://127.0.0.1:20200/conf")
        .header("content-type", "application/json")
        .json(&ReqStruct { app_name })
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    println!("{:?}", response);
    Ok(response)
}

#[cfg(test)]
mod tests {

    use crate::{Conf, ReqStruct, fetch_config_from_server, get_config};
    use mockito::Matcher::Json;
    use serde_json::json;

    fn init_test_conf() -> Conf {
        Conf {
            port: "test".to_string(),
            host: "test".to_string(),
            pg_db_connection_port: "test".to_string(),
            pg_db_host: "test".to_string(),
            pg_db_name: "test".to_string(),
            pg_db_password: "test".to_string(),
            pg_db_username: "test".to_string(),
        }
    }

    #[tokio::test]
    async fn fetch_config_from_server_test() {
        let response_body = serde_json::to_string(&init_test_conf()).unwrap();
        let opts = mockito::ServerOpts {
            host: "127.0.0.1",
            port: 20200,
            ..Default::default()
        };
        println!("{}", response_body);
        let mut server = mockito::Server::new_with_opts(opts);

        let _mock = server
            .mock("POST", "/conf")
            .match_body(Json(json!(ReqStruct {
                app_name: "test".to_string()
            })))
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body(response_body)
            .create();
        let x = fetch_config_from_server("test".to_string()).await.unwrap();
        _mock.assert();
        assert!(x == init_test_conf(), "true")
    }

    #[tokio::test]
    async fn get_config_test() {
        let x = get_config("test".to_string()).await.unwrap();
        assert!(x == init_test_conf(), "true")
    }
}
