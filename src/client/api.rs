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

macro_rules! read_u64 {
    ($name:ident) => {
        println!(stringify!($name));
        let $name = match read_string() {
            Some($name) => match $name.parse::<u64>() {
                Ok($name) => $name,
                Err(_) => {
                    verdict(false, Some(&format!("Failed to parse argument: {}", stringify!($name))));
                    return;
                }
            },
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
    read_u64!(uid);
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

#[inline]
pub async fn user_borrowed(client: &Client) {
    read_u64!(uid);
    let response = client.get("user/borrowed", [
        ("uid", &uid.to_string()),
    ]).await;
    let response: ResponseUserBorrowed = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    let iid_list = response.iid_list;
    value("iid_list", iid_list.len());
}

#[inline]
pub async fn user_reserved(client: &Client) {
    read_u64!(uid);
    let response = client.get("user/reserved", [
        ("uid", &uid.to_string()),
    ]).await;
    let response: ResponseUserReserved = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    let iid_list = response.iid_list;
    value("iid_list", iid_list.len());
}

#[inline]
pub async fn user_unregister(client: &Client) {
    read_u64!(uid);
    let response = client.get("user/unregister", [
        ("uid", &uid.to_string()),
    ]).await;
    let response: ResponseUserUnregister = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}

#[inline]
pub async fn user_borrow(client: &Client) {
    read_u64!(uid);
    read_u64!(iid);
    let response = client.get("user/borrow", [
        ("uid", &uid.to_string()),
        ("iid", &iid.to_string()),
    ]).await;
    let response: ResponseBookBorrow = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}

#[inline]
pub async fn user_return(client: &Client) {
    read_u64!(uid);
    read_u64!(iid);
    let response = client.get("user/return", [
        ("uid", &uid.to_string()),
        ("iid", &iid.to_string()),
    ]).await;
    let response: ResponseBookReturn = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}

#[inline]
pub async fn user_reserve(client: &Client) {
    read_u64!(uid);
    read_u64!(iid);
    let response = client.get("user/reserve", [
        ("uid", &uid.to_string()),
        ("iid", &iid.to_string()),
    ]).await;
    let response: ResponseBookReserve = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}

#[inline]
pub async fn user_info(client: &Client) {
    read_u64!(uid);
    let response = client.get("user/info", [
        ("uid", &uid.to_string()),
    ]).await;
    let response: ResponseUserInfo = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
    value("username", response.username);
    value("email", response.email);
    value("info", response.info);
}

#[inline]
pub async fn admin_add(client: &Client) {
    read_arg!(title);
    read_arg!(author);
    read_arg!(info);
    let request = RequestBookAdd {
        title,
        author,
        info,
    };
    let response = client.post("admin/add", request).await;
    let response: ResponseBookAdd = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    if response.success {
        value("bid", response.bid);
    }
}

#[inline]
pub async fn admin_remove(client: &Client) {
    read_u64!(bid);
    let response = client.get("admin/remove", [
        ("bid", &bid.to_string()),
    ]).await;
    let response: ResponseBookRemove = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
}

#[inline]
pub async fn admin_alter(client: &Client) {
    read_u64!(bid);
    read_arg!(title);
    read_arg!(author);
    read_arg!(info);
    let request = RequestBookAlter {
        bid,
        title,
        author,
        info,
    };
    let response = client.post("admin/alter", request).await;
    let response: ResponseBookAlter = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
}

#[inline]
pub async fn admin_add_instance(client: &Client) {
    read_u64!(bid);
    read_u64!(status);
    let request = RequestBookAddInstance {
        bid,
        status,
    };
    let response = client
        .post("admin/add_instance", request).await;
    let response: ResponseBookAddInstance = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    verdict(response.success, Some(&response.message));
    if response.success {
        value("iid", response.iid);
    }
}

#[inline]
pub async fn admin_remove_instance(client: &Client) {
    read_u64!(iid);
    let response = client.get("admin/remove_instance", [
        ("iid", &iid.to_string()),
    ]).await;
    let response: ResponseBookRemoveInstance = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
}

#[inline]
pub async fn book_search(client: &Client) {
    read_arg!(phrase);
    let response = client.get("book/search", [
        ("phrase", &phrase),
    ]).await;
    let response: ResponseBookSearch = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
    value("bid_list", response.bid_list);
}

#[inline]
pub async fn book_info(client: &Client) {
    read_u64!(bid);
    let response = client.get("book/info", [
        ("bid", &bid.to_string()),
    ]).await;
    let response: ResponseBookInfo = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
    value("title", response.title);
    value("author", response.author);
    value("info", response.info);
}

#[inline]
pub async fn book_instance(client: &Client) {
    read_u64!(bid);
    let response = client.get("book/instance", [
        ("bid", &bid.to_string()),
    ]).await;
    let response: ResponseBookInstance = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
    value("iid_list", response.iid_list);
}

#[inline]
pub async fn book_instance_info(client: &Client) {
    read_u64!(iid);
    let response = client.get("book/instance_info", [
        ("iid", &iid.to_string()),
    ]).await;
    let response: ResponseBookInstanceInfo = match response {
        Some(response) => response,
        None => {
            verdict(false, Some("Failed to receive response"));
            return;
        }
    };
    if !response.success {
        verdict(false, Some(&response.message));
        return;
    }
    verdict(true, None);
    value("bid", response.bid);
    value("status", response.status);
}
