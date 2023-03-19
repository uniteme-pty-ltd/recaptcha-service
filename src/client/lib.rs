// use reqwest::StatusCode;
// use serde::{Deserialize, Serialize};

// pub mod routes;

// #[derive(Clone)]
// pub struct Client {
//     host: String,
// }

// impl Client {
//     pub fn new(host: String) -> Client {
//         Client { host: host }
//     }

//     fn endpoint(&self, url: &str, values: Vec<String>) -> String {
//         let segments: Vec<&str> = url.split("{}").collect();

//         let mut endpoint = String::new();
//         endpoint.push_str(&self.host);

//         for index in 0..segments.len() {
//             if index > 0 {
//                 endpoint.push_str(values.get(index - 1).expect("Not enough values provided"));
//             }
//             endpoint.push_str(segments.get(index).unwrap());
//         }

//         endpoint
//     }
// }

// fn req() -> reqwest::Client {
//     reqwest::Client::new()
// }
