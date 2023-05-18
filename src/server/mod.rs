mod user;
mod admin;
mod book;

use user::*;
use admin::*;
use book::*;

use std::collections::HashMap;
use log::info;
use rusqlite::Connection;
use warp::Filter;
use crate::model::*;

const SERVER_README: &str = include_str!("../../server_readme.txt");

macro_rules! endpoint_debug {
    () => {
        warp::query::<HashMap<String, String>>()
            .map(|query: HashMap<String, String>| {
                log::debug!("Debugging endpoint hit.");
                let mut result = String::new();
                for (key, value) in query {
                    result.push_str(&format!("{}: {}\n", key, value));
                }
                result
            })
    };
}

macro_rules! endpoint_post_request {
    ($name:tt, $callback:ident) => {
        warp::path($name)
            .and(warp::path::end())
            .and(warp::post())
            .and(warp::body::content_length_limit(1024 * 16))
            .and(warp::body::json())
            .map(|req| warp::reply::json(&$callback(req)))
    };
}

macro_rules! endpoint_get_request {
    ($name:tt, $callback:ident) => {
        warp::path($name)
            .and(warp::path::end())
            .and(warp::get())
            .and(warp::query())
            .map(|req| warp::reply::json(&$callback(req)))
    };
}

struct Server {
    database: Connection

}

pub async fn main_server(port: String) {
    env_logger::init();
    info!("Library Management Service by Midnight233, Version {}", env!("CARGO_PKG_VERSION"));

    let root = warp::path::end()
        .map(move || format!(
            "Library Management Service by Midnight233, Version {}\n\n{}",
            env!("CARGO_PKG_VERSION"), SERVER_README));

    let user = {
        let user = warp::path("user");
        let register = endpoint_post_request!("register", user_register);
        let name_lookup = endpoint_get_request!("name_lookup", user_name_lookup);
        let email_lookup = endpoint_get_request!("email_lookup", user_email_lookup);
        let alter_name = endpoint_post_request!("alter_name", )
        user.and(register
            .or(name_lookup)
            .or(email_lookup))
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
        .or(user);

    warp::serve(api)
        .run(([127, 0, 0, 1], port.parse::<u16>().unwrap()))
        .await;
}