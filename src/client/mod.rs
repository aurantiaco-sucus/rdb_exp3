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

    async fn get<const N: usize, ResTy: DeserializeOwned>(
        &self,
        path: &str,
        query: [(&str, &str); N],
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

pub async fn main_client(host: String, port: String) {
    let client = Client::new(host, port);
    while let (Some(category), Some(function)) = (read_line(), read_line()) {
        match category.as_str() {
            "user" => match function.as_str() {
                "register" => user_register(&client).await,
                "lookup" => user_lookup(&client).await,
                "alter" => user_alter(&client).await,
                "borrowed" => user_borrowed(&client).await,
                "reserved" => user_reserved(&client).await,
                "unregister" => user_unregister(&client).await,
                "borrow" => user_borrow(&client).await,
                "reserve" => user_reserve(&client).await,
                "return" => user_return(&client).await,
                "info" => user_info(&client).await,
                _ => println!("unknown function: {}", function),
            },
            "book" => match function.as_str() {
                "search" => book_search(&client).await,
                "info" => book_info(&client).await,
                "instance" => book_instance(&client).await,
                "instance_info" => book_instance_info(&client).await,
                _ => println!("unknown function: {}", function),
            },
            "admin" => match function.as_str() {
                "add" => admin_add(&client).await,
                "remove" => admin_remove(&client).await,
                "alter" => admin_alter(&client).await,
                "add_instance" => admin_add_instance(&client).await,
                "remove_instance" => admin_remove_instance(&client).await,
                _ => println!("unknown function: {}", function),
            }
            _ => println!("unknown category: {}", category),
        }
    }
}