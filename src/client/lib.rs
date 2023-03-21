use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub mod routes;

#[derive(Clone)]
pub struct Client {
    // The host of the recaptcha_service
    host: String,
    secret: String,
    // The hostname of the domain that the recaptcha should've been completed from
    hostname: Option<String>,
}

impl Client {
    pub fn new(host: String, secret: String, hostname: Option<String>) -> Client {
        Client {
            host,
            secret,
            hostname,
        }
    }

    fn endpoint(&self, url: &str, values: Vec<String>) -> String {
        let segments: Vec<&str> = url.split("{}").collect();

        let mut endpoint = String::new();
        endpoint.push_str(&self.host);

        for index in 0..segments.len() {
            if index > 0 {
                endpoint.push_str(values.get(index - 1).expect("Not enough values provided"));
            }
            endpoint.push_str(segments.get(index).unwrap());
        }

        endpoint
    }
}

fn req() -> reqwest::Client {
    reqwest::Client::new()
}
