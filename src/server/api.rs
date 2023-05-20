use log::{info};
use crate::model::*;
use crate::server::database;
use crate::utils::*;

#[inline]
pub fn user_register(req: RequestUserRegister) -> ResponseUserRegister {
    info!("user_register IN {:?}", req);
    if !is_username_legit(&req.username) {
        info!("user_register ERR username is not legit");
        return ResponseUserRegister {
            success: false,
            uid: 0,
            message: "username is not legit".to_string(),
        };
    }
    if !is_email_legit(&req.email) {
        info!("user_register ERR email is not legit");
        return ResponseUserRegister {
            success: false,
            uid: 0,
            message: "email is not legit".to_string(),
        };
    }
    let db = database();
    if let Err(err) = db.execute(
        "INSERT INTO lms_user (username, email, info) VALUES (?1, ?2, ?3)",
        [&req.username, &req.email, &req.info],
    ) {
        info!("user_register ERR {:?}", err);
        ResponseUserRegister {
            success: false,
            uid: 0,
            message: format!("{}", err),
        }
    } else {
        let uid = db.last_insert_rowid() as u64;
        info!("user_register OUT {}", uid);
        ResponseUserRegister {
            success: true,
            uid,
            message: "success".to_string(),
        }
    }
}

#[inline]
pub fn user_lookup(req: RequestUserLookup) -> ResponseUserLookup {
    info!("user_lookup IN {:?}", req);
    let db = database();
    let (phrase, query) = if req.phrase.starts_with(':') {
        (&req.phrase[1..], "SELECT uid FROM lms_user WHERE email = ?1")
    } else {
        (req.phrase.as_str(), "SELECT uid FROM lms_user WHERE username = ?1")
    };
    let uid = db.query_row(query, [phrase], |row| row.get(0));
    match uid {
        Ok(uid) => {
            info!("user_lookup OUT {:?}", uid);
            ResponseUserLookup {
                success: true,
                uid,
                message: "success".to_string(),
            }
        }
        Err(err) => {
            info!("user_lookup ERR {:?}", err);
            ResponseUserLookup {
                success: false,
                uid: 0,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_alter(req: RequestUserAlter) -> ResponseUserAlter {
    info!("user_alter IN {:?}", req);
    if !is_username_legit(&req.username) {
        info!("user_alter ERR username is not legit");
        return ResponseUserAlter {
            success: false,
            message: "username is not legit".to_string(),
        };
    }
    let res = database().execute(
        "UPDATE lms_user SET username = ?1, email = ?2, info = ?3 WHERE uid = ?4",
        [&req.username, &req.email, &req.info, &req.uid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_alter_name OUT {:?}", req);
            ResponseUserAlter {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_alter_name ERR {:?}", err);
            ResponseUserAlter {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_borrowed(req: RequestUserBorrowed) -> ResponseUserBorrowed {
    info!("user_borrowed IN {:?}", req);
    let db = database();
    let mut stmt = db.prepare(
        "SELECT iid FROM lms_occupation WHERE uid = ?1 AND kind = 0",
    ).unwrap();
    let iid_iter = stmt
        .query_map([&req.uid.to_string()], |row| row.get(0));
    let iid_iter = match iid_iter {
        Ok(iid_iter) => iid_iter,
        Err(err) => {
            info!("user_borrowed ERR {:?}", err);
            return ResponseUserBorrowed {
                success: false,
                message: format!("{}", err),
                iid_list: String::new(),
            };
        }
    };
    let iid_list = iid_iter
        .map(|iid: Result<u64, _>| iid.unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");
    info!("user_borrowed OUT {:?}", iid_list);
    ResponseUserBorrowed {
        success: true,
        message: "success".to_string(),
        iid_list,
    }
}

#[inline]
pub fn user_reserved(req: RequestUserReserved) -> ResponseUserReserved {
    info!("user_borrowed IN {:?}", req);
    let db = database();
    let mut stmt = db.prepare(
        "SELECT iid FROM lms_occupation WHERE uid = ?1 AND kind = 1",
    ).unwrap();
    let iid_iter = stmt
        .query_map([&req.uid.to_string()], |row| row.get(0));
    let iid_iter = match iid_iter {
        Ok(iid_iter) => iid_iter,
        Err(err) => {
            info!("user_borrowed ERR {:?}", err);
            return ResponseUserReserved {
                success: false,
                message: format!("{}", err),
                iid_list: String::new(),
            };
        }
    };
    let iid_list = iid_iter
        .map(|iid: Result<u64, _>| iid.unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");
    info!("user_borrowed OUT {:?}", iid_list);
    ResponseUserReserved {
        success: true,
        message: "success".to_string(),
        iid_list,
    }
}

#[inline]
pub fn user_unregister(req: RequestUserUnregister) -> ResponseUserUnregister {
    info!("user_unregister IN {:?}", req);
    let res = database().execute(
        "DELETE FROM lms_user WHERE uid = ?1",
        [&req.uid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_unregister OUT {:?}", req);
            ResponseUserUnregister {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_unregister ERR {:?}", err);
            ResponseUserUnregister {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_borrow(req: RequestBookBorrow) -> ResponseBookBorrow {
    info!("user_borrow IN {:?}", req);
    let res = database().execute(
        "INSERT INTO lms_occupation (uid, iid, date, kind) VALUES (?1, ?2, date('now'), 0)",
        [&req.uid.to_string(), &req.iid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_borrow OUT {:?}", req);
            ResponseBookBorrow {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_borrow ERR {:?}", err);
            ResponseBookBorrow {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_return(req: RequestBookReturn) -> ResponseBookReturn {
    info!("user_return IN {:?}", req);
    let res = database().execute(
        "DELETE FROM lms_occupation WHERE iid = ?2",
        [&req.iid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_return OUT {:?}", req);
            ResponseBookReturn {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_return ERR {:?}", err);
            ResponseBookReturn {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_reserve(req: RequestBookReserve) -> ResponseBookReserve {
    info!("user_reserve IN {:?}", req);
    let db = database();
    let res = db.execute(
        "INSERT INTO lms_occupation (uid, iid, date, kind) VALUES (?1, ?2, date('now'), ?3)",
        [&req.uid.to_string(), &req.iid.to_string(), &1.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_reserve OUT {:?}", req);
            ResponseBookReserve {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_reserve ERR {:?}", err);
            ResponseBookReserve {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_info(req: RequestUserInfo) -> ResponseUserInfo {
    info!("user_info IN {:?}", req);
    let res = database().query_row(
        "SELECT username, email, info FROM lms_user WHERE uid = ?1",
        [&req.uid.to_string()],
        |row| {
            Ok((
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            ))
        }
    );
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            info!("user_info ERR {:?}", err);
            return ResponseUserInfo {
                success: false,
                message: format!("{}", err),
                username: String::new(),
                email: String::new(),
                info: String::new(),
            };
        }
    };
    info!("user_info OUT {:?}", req);
    ResponseUserInfo {
        success: true,
        message: "success".to_string(),
        username: res.0,
        email: res.1,
        info: res.2,
    }
}

#[inline]
pub fn admin_add(req: RequestBookAdd) -> ResponseBookAdd {
    info!("admin_add IN {:?}", req);
    let db = database();
    let res = db.execute(
        "INSERT INTO lms_book (title, author, info) VALUES (?1, ?2, ?3)",
        [&req.title, &req.author, &req.info],
    );
    match res {
        Ok(_) => {
            let bid = db.last_insert_rowid() as u64;
            info!("admin_add OUT {:?}", req);
            ResponseBookAdd {
                success: true,
                bid,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("admin_add ERR {:?}", err);
            ResponseBookAdd {
                success: false,
                bid: 0,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn admin_remove(req: RequestBookRemove) -> ResponseBookRemove {
    info!("admin_remove IN {:?}", req);
    let res = database().execute(
        "DELETE FROM lms_book WHERE bid = ?1",
        [&req.bid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("admin_remove OUT {:?}", req);
            ResponseBookRemove {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("admin_remove ERR {:?}", err);
            ResponseBookRemove {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn admin_alter(req: RequestBookAlter) -> ResponseBookAlter {
    info!("admin_alter IN {:?}", req);
    let res = database().execute(
        "UPDATE lms_book SET title = ?1, author = ?2, info = ?3 WHERE bid = ?5",
        [&req.title, &req.author, &req.info, &req.bid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("admin_alter OUT {:?}", req);
            ResponseBookAlter {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("admin_alter ERR {:?}", err);
            ResponseBookAlter {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn admin_add_instance(req: RequestBookAddInstance) -> ResponseBookAddInstance {
    info!("admin_add_instance IN {:?}", req);
    let db = database();
    let res = db.execute(
        "INSERT INTO lms_instance (bid, status) VALUES (?1, ?2)",
        [&req.bid.to_string(), &req.status.to_string()],
    );
    match res {
        Ok(_) => {
            let iid = db.last_insert_rowid() as u64;
            info!("admin_add_instance OUT {iid}");
            ResponseBookAddInstance {
                success: true,
                message: "success".to_string(),
                iid,
            }
        },
        Err(err) => {
            info!("admin_add_instance ERR {:?}", err);
            ResponseBookAddInstance {
                success: false,
                message: format!("{}", err),
                iid: 0,
            }
        }
    }
}

#[inline]
pub fn admin_remove_instance(req: RequestBookRemoveInstance) -> ResponseBookRemoveInstance {
    info!("admin_remove_instance IN {:?}", req);
    let res = database().execute(
        "DELETE FROM lms_instance WHERE iid = ?1",
        [&req.iid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("admin_remove_instance OUT {:?}", req);
            ResponseBookRemoveInstance {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("admin_remove_instance ERR {:?}", err);
            ResponseBookRemoveInstance {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn book_search(req: RequestBookSearch) -> ResponseBookSearch {
    info!("book_search IN {:?}", req);
    let db = database();
    let mut stmt = db.prepare(
        "SELECT bid FROM lms_book WHERE \
        title LIKE '%?1%' OR \
        author LIKE '%?1%' OR \
        info LIKE '%?1%'",
    ).unwrap();
    let bids = stmt.query_map(
        [&req.phrase],
        |row| {
            Ok(row.get(0).unwrap())
        }
    );
    let bids = match bids {
        Ok(bids) => bids,
        Err(err) => {
            info!("book_search ERR {:?}", err);
            return ResponseBookSearch {
                success: false,
                message: format!("{}", err),
                bid_list: String::new(),
            };
        }
    };
    let bids = bids
        .map(|bid| bid.unwrap())
        .collect::<Vec<String>>();
    info!("book_search OUT {:?}", req);
    ResponseBookSearch {
        success: true,
        message: "success".to_string(),
        bid_list: bids.join(","),
    }
}

#[inline]
pub fn book_info(req: RequestBookInfo) -> ResponseBookInfo {
    info!("book_info IN {:?}", req);
    let res = database().query_row(
        "SELECT title, author, info FROM lms_book WHERE bid = ?1",
        [&req.bid.to_string()],
        |row| {
            Ok((
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
            ))
        }
    );
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            info!("book_info ERR {:?}", err);
            return ResponseBookInfo {
                success: false,
                message: format!("{}", err),
                title: String::new(),
                author: String::new(),
                info: String::new(),
            };
        }
    };
    info!("book_info OUT {:?}", req);
    ResponseBookInfo {
        success: true,
        message: "success".to_string(),
        title: res.0,
        author: res.1,
        info: res.2,
    }
}

#[inline]
pub fn book_instance(req: RequestBookInstance) -> ResponseBookInstance {
    info!("book_instance IN {:?}", req);
    let db = database();
    let mut stmt = db
        .prepare("SELECT iid FROM lms_instance WHERE bid = ?1")
        .unwrap();
    let iid_list = stmt.query_map(
        [&req.bid.to_string()],
        |row| {
            Ok(row.get(0).unwrap())
        });
    let iid_list = match iid_list {
        Ok(iid_list) => iid_list,
        Err(err) => {
            info!("book_instance ERR {:?}", err);
            return ResponseBookInstance {
                success: false,
                message: format!("{}", err),
                iid_list: String::new(),
            };
        }
    };
    let iid_list = iid_list
        .map(|iid| iid.unwrap())
        .collect::<Vec<String>>();
    info!("book_instance OUT {:?}", req);
    ResponseBookInstance {
        success: true,
        message: "success".to_string(),
        iid_list: iid_list.join(","),
    }
}

#[inline]
pub fn book_instance_info(req: RequestBookInstanceInfo) -> ResponseBookInstanceInfo {
    info!("book_instance_info IN {:?}", req);
    let res = database().query_row(
        "SELECT bid, status FROM lms_instance WHERE iid = ?1",
        [&req.iid.to_string()],
        |row| {
            Ok((
                row.get(0).unwrap(),
                row.get(1).unwrap(),
            ))
        }
    );
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            info!("book_instance_info ERR {:?}", err);
            return ResponseBookInstanceInfo {
                success: false,
                message: format!("{}", err),
                bid: 0,
                status: 0,
            };
        }
    };
    info!("book_instance_info OUT {:?}", req);
    ResponseBookInstanceInfo {
        success: true,
        message: "success".to_string(),
        bid: res.0,
        status: res.1,
    }
}