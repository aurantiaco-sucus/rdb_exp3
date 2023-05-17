use std::collections::HashMap;
use log::info;
use warp::Filter;

const DOCUMENT: &str = include_str!("document.txt");

#[macro_export]
macro_rules! endpoint_debug {
    () => {
        warp::query::<HashMap<String, String>>()
            .map(|query: HashMap<String, String>| {
                let mut result = String::new();
                for (key, value) in query {
                    result.push_str(&format!("{}: {}\n", key, value));
                }
                result
            })
    };
}

#[tokio::main]
async fn main() {
    let lms_launch_type = std::env::var("LMS_LAUNCH_TYPE")
        .unwrap_or_else(|_| "client".to_string());
    let lms_host = std::env::var("LMS_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let lms_port = std::env::var("LMS_PORT")
        .unwrap_or_else(|_| "9998".to_string());
    match lms_launch_type.as_str() {
        "server" => main_server(lms_port).await,
        "client" => main_client(lms_host, lms_port).await,
        _ => panic!("Unknown launch type: {}", lms_launch_type),
    }
}

async fn main_server(port: String) {
    env_logger::init();
    info!("Library Management Service by Midnight233, Version {}", env!("CARGO_PKG_VERSION"));

    let root = warp::path::end()
        .map(move || format!(
            "Library Management Service by Midnight233, Version {}\n\n{}",
            env!("CARGO_PKG_VERSION"), DOCUMENT));

    let user = {
        let user = warp::path("user");
        let register = user
            .and(warp::path("register"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let search = user
            .and(warp::path("search"))
            .and(warp::path::end())
            .and(warp::get())
            .and(endpoint_debug!());
        let history = user
            .and(warp::path("history"))
            .and(warp::path::end())
            .and(warp::get())
            .and(endpoint_debug!());
        user.and(register
            .or(search)
            .or(history))
    };

    let book = {
        let book = warp::path("book");
        let borrow = book
            .and(warp::path("borrow"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let _return = book
            .and(warp::path("return"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let renew = book
            .and(warp::path("renew"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        book.and(borrow
            .or(_return)
            .or(renew))
    };

    let admin = {
        let admin = warp::path("admin");
        let add = admin
            .and(warp::path("add"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let delete = admin
            .and(warp::path("delete"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let modify = admin
            .and(warp::path("modify"))
            .and(warp::path::end())
            .and(warp::post())
            .and(endpoint_debug!());
        let search = admin
            .and(warp::path("search"))
            .and(warp::path::end())
            .and(warp::get())
            .and(endpoint_debug!());
        admin.and(add
            .or(delete)
            .or(modify)
            .or(search))
    };

    let api = root
        .or(user)
        .or(book)
        .or(admin);

    warp::serve(api)
        .run(([127, 0, 0, 1], port.parse::<u16>().unwrap()))
        .await;
}

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

async fn main_client(host: String, port: String) {
    let client = Client::new(host, port);
}