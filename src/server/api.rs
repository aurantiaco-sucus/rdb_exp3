use log::{info};
use crate::model::*;
use crate::server::database;

/*
In case you forget about it, here is how the tables look like:

create table lms_user (
    uid integer primary key autoincrement,
    username text not null,
    email text not null,
    info text not null
);

create index lms_user_username on lms_user (username);
create index lms_user_email on lms_user (email);

create table lms_book (
    bid integer primary key autoincrement,
    title text not null,
    author text not null,
    info text not null
);

create table lms_instance (
    iid integer primary key autoincrement,
    bid integer not null,
    foreign key (bid) references lms_book (bid)
);

create index lms_instance_bid on lms_instance (bid);

create table lms_occupation (
    uid integer not null,
    iid integer not null,
    date text not null,
    kind integer not null,
    foreign key (uid) references lms_user (uid),
    foreign key (iid) references lms_instance (iid),
    primary key (uid, iid),
    check (kind in (0, 1, 2)) -- 0: borrowed, 1: reserved, 2: lost
);

create index lms_borrow_uid on lms_occupation (uid);
create index lms_borrow_iid on lms_occupation (iid);
 */

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
pub fn is_username_legit(username: &str) -> bool {
    username.len() >= 8 &&
        username.len() <= 128 &&
        username.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
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
        .query_map(&[&req.uid.to_string()], |row| row.get(0));
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
        .query_map(&[&req.uid.to_string()], |row| row.get(0));
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
    let date = chrono::Local::now().date_naive();
    let res = database().execute(
        "INSERT INTO lms_borrow (uid, iid, date, kind) VALUES (?1, ?2, ?3, 0)",
        [&req.uid.to_string(), &req.iid.to_string(), &date.to_string()],
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
    let date = chrono::Local::now().date_naive();
    let res = database().execute(
        "DELETE FROM lms_borrow WHERE uid = ?1 AND iid = ?2",
        [&req.uid.to_string(), &req.iid.to_string()],
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
pub fn admin_add(req: RequestBookAdd) -> ResponseBookAdd {
    info!("admin_add IN {:?}", req);
    // title, author, description and copies
    let res = database().execute(
        "INSERT INTO lms_book (title, author, description, copies) VALUES (?1, ?2, ?3, ?4)",
        [&req.title, &req.author, &req.description, &req.copies.to_string()],
    );
    match res {
        Ok(_) => {
            let bid = database().query_row(
                "SELECT bid FROM lms_book WHERE title = ?1",
                [&req.title],
                |row| {
                    Ok(row.get(0).unwrap())
                }
            );
            let bid = match bid {
                Ok(bid) => bid,
                Err(err) => {
                    info!("admin_add ERR {:?}", err);
                    return ResponseBookAdd {
                        success: false,
                        message: format!("{}", err),
                        bid: 0,
                    };
                }
            };
            info!("admin_add OUT {:?}", req);
            ResponseBookAdd {
                success: true,
                message: "success".to_string(),
                bid,
            }
        },
        Err(err) => {
            info!("admin_add ERR {:?}", err);
            ResponseBookAdd {
                success: false,
                message: format!("{}", err),
                bid: 0,
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
        "UPDATE lms_book SET title = ?1, author = ?2, description = ?3 WHERE bid = ?5",
        [&req.title, &req.author, &req.description, &req.bid.to_string()],
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
pub fn admin_alter_copies(req: RequestBookAlterCopies) -> ResponseBookAlterCopies {
    info!("admin_alter_copies IN {:?}", req);
    let res = database().execute(
        "UPDATE lms_book SET copies = ?1 WHERE bid = ?2",
        [&req.copies.to_string(), &req.bid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("admin_alter_copies OUT {:?}", req);
            ResponseBookAlterCopies {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("admin_alter_copies ERR {:?}", err);
            ResponseBookAlterCopies {
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
        description LIKE '%?1%'",
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
                bids: String::new(),
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
        bids: bids.join(","),
    }
}

#[inline]
pub fn book_info(req: RequestBookInfo) -> ResponseBookInfo {
    info!("book_info IN {:?}", req);
    let res = database().query_row(
        "SELECT title, author, description, copies, available FROM lms_book WHERE bid = ?1",
        [&req.bid.to_string()],
        |row| {
            Ok((
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
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
                description: String::new(),
                copies: 0,
                available: 0,
            };
        }
    };
    info!("book_info OUT {:?}", req);
    ResponseBookInfo {
        success: true,
        message: "success".to_string(),
        title: res.0,
        author: res.1,
        description: res.2,
        copies: res.3,
        available: res.4,
    }
}