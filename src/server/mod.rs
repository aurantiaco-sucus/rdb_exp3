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
    // check if tables `lms_user`, `lms_book` and `lms_borrow` exist
    ["lms_user", "lms_book", "lms_borrow"].iter().for_each(|table| {
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
        let name_lookup = endpoint_get_request!("name_lookup", user_name_lookup);
        let email_lookup = endpoint_get_request!("email_lookup", user_email_lookup);
        let alter_name = endpoint_post_request!("alter_name", user_alter_name);
        let alter_email = endpoint_post_request!("alter_email", user_alter_email);
        let borrowed = endpoint_get_request!("borrowed", user_borrowed);
        let unregister = endpoint_post_request!("unregister", user_unregister);
        let borrow = endpoint_post_request!("borrow", user_borrow);
        let return_ = endpoint_post_request!("return", user_return);
        warp::path("user").and(register
            .or(name_lookup)
            .or(email_lookup)
            .or(alter_name)
            .or(alter_email)
            .or(borrowed)
            .or(unregister)
            .or(borrow)
            .or(return_))
    };

    let book = {
        let search = endpoint_get_request!("search", book_search);
        let info = endpoint_get_request!("info", book_info);
        warp::path("book").and(search
            .or(info))
    };

    let admin = {
        let add = endpoint_post_request!("add", admin_add);
        let remove = endpoint_post_request!("remove", admin_remove);
        let alter = endpoint_post_request!("alter", admin_alter);
        let alter_copies = endpoint_post_request!("alter_copies", admin_alter_copies);
        warp::path("admin").and(add
            .or(remove)
            .or(alter)
            .or(alter_copies))
    };

    let api = root
        .or(user)
        .or(book)
        .or(admin);

    warp::serve(api)
        .run(([127, 0, 0, 1], port.parse::<u16>().unwrap()))
        .await;
}