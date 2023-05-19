use std::fmt::Display;
use crate::client::{Client, read_string};
use crate::model::*;
use crate::utils::*;

macro_rules! read_arg {
    ($name:ident) => {
        println!(stringify!($name));
        let $name = match read_string() {
            Some($name) => $name,
            None => {
                verdict(false, Some(&format!("Failed to read argument: {}", stringify!($name))));
                return;
            }
        };
    };
}

#[inline]
fn verdict(success: bool, message: Option<&str>) {
    println!("{}", if success { "OK" } else { "ERR" });
    if let Some(message) = message {
        println!("{message}");
    }
}

#[inline]
fn value(key: &str, val: impl Display) {
    println!("{key}={val}");
}

#[inline]
pub async fn user_register(client: &Client) {
    read_arg!(username);
    if !is_username_legit(&username) {
        verdict(false, Some("Username is not legit"));
        return;
    }
    read_arg!(email);
    if !is_email_legit(&email) {
        verdict(false, Some("Email is not legit"));
        return;
    }
    read_arg!(info);
    let request = RequestUserRegister {
        username,
        email,
        info,
    };
    let response = client.post("user/register", request).await;
    let response: ResponseUserRegister = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    if response.success {
        value("uid", response.uid);
    }
}

#[inline]
pub async fn user_lookup(client: &Client) {
    read_arg!(phrase);
    let response = client.get("user/lookup", [
        ("phrase", &phrase),
    ]).await;
    let response: ResponseUserLookup = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    if response.success {
        value("uid", response.uid);
    }
}

#[inline]
pub async fn user_alter(client: &Client) {
    read_arg!(uid);
    let uid = match uid.parse::<u64>() {
        Ok(uid) => uid,
        Err(_) => {
            verdict(false, Some("Failed to parse uid"));
            return;
        }
    };
    read_arg!(username);
    if !is_username_legit(&username) {
        verdict(false, Some("Username is not legit"));
        return;
    }
    read_arg!(email);
    if !is_email_legit(&email) {
        verdict(false, Some("Email is not legit"));
        return;
    }
    read_arg!(info);
    let request = RequestUserAlter {
        uid,
        username,
        email,
        info,
    };
    let response = client.post("user/alter", request).await;
    let response: ResponseUserAlter = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}