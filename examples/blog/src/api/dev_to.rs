// https://github.com/rust-lang-nursery/lazy-static.rs

use lazy_static::lazy_static;
use dotenvy::dotenv;
use std::env;
// use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

lazy_static! {
    pub static ref DEV_TO_CLIENT: Client = {
        dotenv().expect("No .env file");
        let dev_to = env::var("DEV_TO").unwrap();
        // println!("dev_to {:#?}", dev_to);

        let mut headers = HeaderMap::new();
        let mut auth_value = HeaderValue::from_str(&dev_to).unwrap();
        auth_value.set_sensitive(true);
        headers.insert("api-key", auth_value);

        headers.insert("User-Agent", HeaderValue::from_static("blog"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("accept", HeaderValue::from_static("application/json"));

        let dev_to_client = Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        dev_to_client
    };
}

// pub fn test() -> Client {
//     dotenv().expect("No .env file");

//     let mut headers = HeaderMap::new();
//     headers.insert("api-key", HeaderValue::from_str(dotenv!("DEV_TO")).unwrap());

//     let dev_to_client = Client::builder()
//         .default_headers(headers)
//         .base_url("https://dev.to/api")
//         .build()
//         .unwrap();

//     dev_to_client
// }
