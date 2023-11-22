use dotenv::dotenv;
use md5;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use std::env;
use std::env::VarError;
use std::time::Duration;

#[derive()]
pub struct Credentials {
    pub api_key: String,
    pub api_secret: String,
    pub username: String,
    pub password: String,
    pub api_sig: String,
}

fn signature(
    api_key: &String,
    api_secret: &String,
    username: &String,
    password: &String,
) -> String {
    let input = format!(
        "api_key{}methodauth.getMobileSessionpassword{}username{}{}",
        api_key, password, username, api_secret
    );
    format!("{:x}", md5::compute(input))
}

pub fn auth(credentials: Credentials) -> String {
    let headers = HeaderMap::new();
    let client = Client::new();
    let response = client
        .post("https://ws.audioscrobbler.com/2.0/?method=auth.getMobileSession")
        .timeout(Duration::from_secs(20))
        .query(&[
            ("password", credentials.password),
            ("username", credentials.username),
            ("api_key", credentials.api_key),
            ("api_sig", credentials.api_sig),
        ])
        .headers(headers)
        .send()
        .unwrap();

    response.json().unwrap()
}

pub fn get_credentials() -> Result<Credentials, VarError> {
    dotenv().ok();
    let api_key = env::var("API_KEY")?;
    let api_secret = env::var("API_SECRET")?;
    let username = env::var("API_USERNAME")?;
    let password = env::var("API_PASSWORD")?;
    let api_sig = signature(&api_key, &api_secret, &username, &password);
    println!("{}, {}", username, password);
    Ok(Credentials {
        api_key,
        api_secret,
        username,
        password,
        api_sig,
    })
}
