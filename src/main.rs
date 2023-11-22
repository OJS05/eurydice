mod http;

use dotenv::dotenv;
use http::scrobble::get_credentials;
use std::env;

fn main() {
    dotenv().ok();

    println!("{}", env::var("API_PASSWORD").unwrap());
}
