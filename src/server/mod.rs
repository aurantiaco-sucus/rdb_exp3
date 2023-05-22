mod api;

use api::*;

use log::info;
use rusqlite::Connection;
use std::sync::{Mutex, MutexGuard};
use warp::Filter;

const SERVER_README: &str = include_str!("../../assets/server_readme.txt");

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

static mut DATABASE_CONNECTION: Option<Mutex<Connection>> = None;

pub fn database() -> MutexGuard<'static, Connection> {
    unsafe {
        DATABASE_CONNECTION.as_ref().unwrap().lock().unwrap()
    }
}

pub async fn main_server(port: String) {
    env_logger::init();
    info!("Library Management Service by Midnight233, Version {}", env!("CARGO_PKG_VERSION"));

    info!("Connecting to database");
    unsafe {
        DATABASE_CONNECTION = Some(Mutex::new(Connection::open("rdb_exp3.db")
            .expect("Failed to connect to database. Did you run configuration?")));
    }

    info!("Checking sanity of database");
    ["lms_user", "lms_book", "lms_instance", "lms_occupation", "lms_history"].iter()
        .for_each(|table| {
            if database().query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                [table],
                |row| row.get::<_, i64>(0),
            ).unwrap() != 1 {
                panic!("Table `{}` does not exist", table);
            }
        });

    ctrlc::set_handler(move || {
        info!("Shutting down server");
        let db = unsafe { DATABASE_CONNECTION.take().unwrap() };
        db.into_inner().unwrap().close().unwrap();
        std::process::exit(0);
    }).expect("Failed to register Ctrl-C handler");

    info!("Starting server on port {}", port);
    let root = warp::path::end()
        .map(move || format!(
            "Library Management Service by Midnight233, Version {}\n\n{}",
            env!("CARGO_PKG_VERSION"), SERVER_README));

    let user = {
        let register = endpoint_post_request!("register", user_register);
        let borrowed = endpoint_get_request!("borrowed", user_borrowed);
        let unregister = endpoint_post_request!("unregister", user_unregister);
        let borrow = endpoint_post_request!("borrow", user_borrow);
        let return_ = endpoint_post_request!("return", user_return);
        let lookup = endpoint_get_request!("lookup", user_lookup);
        let alter = endpoint_post_request!("alter", user_alter);
        let reserve = endpoint_post_request!("reserve", user_reserve);
        let reserved = endpoint_get_request!("reserved", user_reserved);
        let info = endpoint_get_request!("info", user_info);
        warp::path("user").and(register
            .or(borrowed)
            .or(unregister)
            .or(borrow)
            .or(return_)
            .or(lookup)
            .or(alter)
            .or(reserve)
            .or(reserved)
            .or(info))
    };

    let book = {
        let search = endpoint_get_request!("search", book_search);
        let info = endpoint_get_request!("info", book_info);
        let instance = endpoint_get_request!("instance", book_instance);
        let instance_info = endpoint_get_request!("instance_info", book_instance_info);
        warp::path("book").and(search
            .or(info)
            .or(instance)
            .or(instance_info))
    };

    let admin = {
        let add = endpoint_post_request!("add", admin_add);
        let remove = endpoint_post_request!("remove", admin_remove);
        let alter = endpoint_post_request!("alter", admin_alter);
        let add_instance = endpoint_post_request!("add_instance", admin_add_instance);
        let remove_instance = endpoint_post_request!("remove_instance", admin_remove_instance);
        let occupy_instance = endpoint_post_request!("occupy_instance", admin_occupy_instance);
        let release_instance = endpoint_post_request!("release_instance", admin_release_instance);
        warp::path("admin").and(add
            .or(remove)
            .or(alter)
            .or(add_instance)
            .or(remove_instance)
            .or(occupy_instance)
            .or(release_instance))
    };

    let api = root
        .or(user)
        .or(book)
        .or(admin);

    warp::serve(api)
        .run(([127, 0, 0, 1], port.parse::<u16>().unwrap()))
        .await;
}