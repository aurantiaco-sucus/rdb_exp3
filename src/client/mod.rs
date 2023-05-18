mod api;

use std::collections::HashMap;

pub struct Client {
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

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn read_string() -> String {
    let mut input = String::new();
    let mut lines = Vec::new();
    loop {
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "" {
            break;
        }
        lines.push(input.to_string());
        input.clear();
    }
    lines.join("\n")
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
                "register" => {}
                "lookup" => {}
                "alter" => {}
                "borrowed" => {}
                "reserved" => {}
                "unregister" => {}
                "borrow" => {}
                "reserve" => {}
                "return" => {}
                _ => println!("unknown function: {}", function),
            },
            "book" => match function.as_str() {
                "search" => {}
                "info" => {}
                "instance" => {}
                "instance_info" => {}
                _ => println!("unknown function: {}", function),
            },
            "admin" => match function.as_str() {
                "add" => {}
                "remove" => {}
                "alter" => {}
                "add_instance" => {}
                "remove_instance" => {}
                _ => println!("unknown function: {}", function),
            }
            _ => println!("unknown category: {}", category),
        }
    }
}