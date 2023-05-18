mod api;
use api::*;

use std::collections::HashMap;

struct Client {
    host: String,
    port: String,
    client: reqwest::Client,
}

impl Client {
    fn new(host: String, port: String) -> Self {
        Self {
            host,
            port,
            client: reqwest::Client::new(),
        }
    }

    async fn request(
        &self,
        path: String,
        method: String,
        query: HashMap<String, String>
    ) -> Option<String> {
        let url = format!("http://{}:{}/{}", self.host, self.port, path);
        let client = &self.client;
        let response = match method.as_str() {
            "GET" => client.get(&url).query(&query).send().await,
            "POST" => client.post(&url).form(&query).send().await,
            _ => panic!("Unknown method: {}", method),
        };
        match response {
            Ok(response) => Some(response.text().await.unwrap()),
            Err(_) => None,
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// --- Lines for a request
// category
// function
// --- For each argument
// value count of lines
// --- For each line
// line content
// --- Input accepted after all values are inputted

pub async fn main_client(host: String, port: String) {
    let client = Client::new(host, port);
    loop {
        let category = read_line();
        let function = read_line();
        match category.as_str() {
            "user" => match function.as_str() {
                "register" => {
                    let mut query = HashMap::new();
                    query.insert("username".to_string(), read_line());
                    query.insert("password".to_string(), read_line());
                    query.insert("email".to_string(), read_line());
                    query.insert("phone".to_string(), read_line());
                    query.insert("address".to_string(), read_line());
                    println!("{}", client.request(
                        "user/register".to_string(),
                        "POST".to_string(),
                        query
                    ).await.unwrap());
                }
                "search" => {
                    let mut query = HashMap::new();
                    query.insert("username".to_string(), read_line());
                    println!("{}", client.request(
                        "user/search".to_string(),
                        "GET".to_string(),
                        query
                    ).await.unwrap());
                }
                "history" => {
                    let mut query = HashMap::new();
                    query.insert("username".to_string(), read_line());
                    println!("{}", client.request(
                        "user/history".to_string(),
                        "GET".to_string(),
                        query
                    ).await.unwrap());
                }
                _ => panic!("Unknown function: {}", function),
            },
            _ => {}
        }
    }
}