mod api;

use serde::{Serialize};
use serde::de::DeserializeOwned;
use crate::client::api::*;

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

    async fn get<const n: usize, ResTy: DeserializeOwned>(
        &self,
        path: &str,
        query: [(&str, &str); n],
    ) -> Option<ResTy> {
        let url = format!("http://{}:{}/{}", self.host, self.port, path);
        let client = &self.client;
        let response = client
            .get(&url)
            .query(query.as_slice())
            .send().await.ok()?;
        response.json::<ResTy>().await.ok()
    }

    async fn post<'a, ReqTy: Serialize, ResTy: DeserializeOwned>(
        &self,
        path: &str,
        req: ReqTy
    ) -> Option<ResTy> {
        let url = format!("http://{}:{}/{}", self.host, self.port, path);
        let client = &self.client;
        let response = client
            .post(&url)
            .json(&req)
            .send().await.ok()?;
        response.json::<ResTy>().await.ok()
    }
}

pub fn read_line() -> Option<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;
    Some(input.trim_end_matches('\n').to_string())
}

pub fn read_string() -> Option<String> {
    Some(read_line()?
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\\", "\\"))
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
    while let (Some(category), Some(function)) = (read_line(), read_line()) {
        match category.as_str() {
            "user" => match function.as_str() {
                "register" => user_register(&client).await,
                "lookup" => user_lookup(&client).await,
                "alter" => user_alter(&client).await,
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