use std::fmt::Display;
use std::io::Write;
use crate::client::{Client, read_string};
use crate::model::*;
use crate::utils::*;

macro_rules! read_arg {
    ($name:ident) => {
        println!(stringify!($name));
        std::io::stdout().flush().unwrap();
        let $name = match read_string() {
            Some($name) => $name,
            None => {
                verdict_err(&format!("Failed to read argument: {}", stringify!($name)));
                return;
            }
        };
    };
}

macro_rules! read_u64 {
    ($name:ident) => {
        println!(stringify!($name));
        std::io::stdout().flush().unwrap();
        let $name = match read_string() {
            Some($name) => match $name.parse::<u64>() {
                Ok($name) => $name,
                Err(_) => {
                    verdict_err(&format!("Failed to parse argument: {}", stringify!($name)));
                    return;
                }
            },
            None => {
                verdict_err(&format!("Failed to read argument: {}", stringify!($name)));
                return;
            }
        };
    };
}

#[inline]
fn verdict_ok() {
    println!("OK");
    std::io::stdout().flush().unwrap();
}

#[inline]
fn verdict_err(message: &str) {
    println!("ERR");
    println!("{message}");
    std::io::stdout().flush().unwrap();
}

#[inline]
fn value(key: &str, val: impl Display) {
    println!("{key}={val}");
    std::io::stdout().flush().unwrap();
}

#[inline]
pub async fn user_register(client: &Client) {
    read_arg!(username);
    if !is_username_legit(&username) {
        verdict_err("Username is not legit");
        return;
    }
    read_arg!(email);
    if !is_email_legit(&email) {
        verdict_err("Email is not legit");
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        value("uid", response.uid);
    } else {
        verdict_err(&response.message);
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        value("uid", response.uid);
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn user_alter(client: &Client) {
    read_u64!(uid);
    read_arg!(username);
    if !is_username_legit(&username) {
        verdict_err("Username is not legit");
        return;
    }
    read_arg!(email);
    if !is_email_legit(&email) {
        verdict_err("Email is not legit");
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        let iid_list = response.iid_list;
        value("iid_list", iid_list.len());
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        let iid_list = response.iid_list;
        value("iid_list", iid_list.len());
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn user_unregister(client: &Client) {
    read_u64!(uid);
    let request = RequestUserUnregister {
        uid,
    };
    let response = client.post("user/unregister", request).await;
    let response: ResponseUserUnregister = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn user_borrow(client: &Client) {
    read_u64!(uid);
    read_u64!(iid);
    let request = RequestBookBorrow {
        uid,
        iid,
    };
    let response = client.post("user/borrow", request).await;
    let response: ResponseBookBorrow = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn user_return(client: &Client) {
    read_u64!(iid);
    let request = RequestBookReturn {
        iid,
    };
    let response = client.post("user/return", request).await;
    let response: ResponseBookReturn = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn user_reserve(client: &Client) {
    read_u64!(uid);
    read_u64!(iid);
    let request = RequestBookReserve {
        uid,
        iid,
    };
    let response = client.post("user/reserve", request).await;
    let response: ResponseBookReserve = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if !response.success {
        verdict_err(&response.message);
        return;
    }
    verdict_ok();
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        value("bid", response.bid);
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn admin_remove(client: &Client) {
    read_u64!(bid);
    let request = RequestBookRemove {
        bid,
    };
    let response = client.post("admin/remove", request).await;
    let response: ResponseBookRemove = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
        value("iid", response.iid);
    } else {
        verdict_err(&response.message);
    }
}

#[inline]
pub async fn admin_remove_instance(client: &Client) {
    read_u64!(iid);
    let request = RequestBookRemoveInstance {
        iid,
    };
    let response = client
        .post("admin/remove_instance", request).await;
    let response: ResponseBookRemoveInstance = match response {
        Some(response) => response,
        None => {
            verdict_err("Failed to receive response");
            return;
        }
    };
    if response.success {
        verdict_ok();
    } else {
        verdict_err(&response.message);
    }
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if !response.success {
        verdict_err(&response.message);
        return;
    }
    verdict_ok();
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if !response.success {
        verdict_err(&response.message);
        return;
    }
    verdict_ok();
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if !response.success {
        verdict_err(&response.message);
        return;
    }
    verdict_ok();
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
            verdict_err("Failed to receive response");
            return;
        }
    };
    if !response.success {
        verdict_err(&response.message);
        return;
    }
    verdict_ok();
    value("bid", response.bid);
    value("status", response.status);
}
