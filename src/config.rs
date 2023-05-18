use log::{info, warn};

pub async fn main_config(lms_config_overwrite: bool) {
    let ow = lms_config_overwrite;
    env_logger::init();
    info!("Running 1st time server configuration");
    if ow {
        warn!("Overwriting existing configuration if any");
    }
    config_database(ow);
}

const QUERY_DB_CREATE: &str = include_str!("../src_sql/config_create.sql");

fn config_database(ow: bool) {
    if ow && std::path::Path::new("rdb_exp3.db").exists() {
        info!("Removing existing database");
        std::fs::remove_file("rdb_exp3.db").unwrap();
    }
    info!("Configuring database");
    let db = rusqlite::Connection::open("rdb_exp3.db").unwrap();
    db.execute_batch(QUERY_DB_CREATE).unwrap();
    db.close().unwrap();
}