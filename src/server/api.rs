use log::{info};
use crate::model::*;
use crate::server::database;

/*
In case you forget about it, here is how the tables look like:

create table lms_user (
    uid integer primary key autoincrement,
    username varchar(512) not null unique,
    email varchar(512) not null unique
);

create table lms_book (
    bid integer primary key autoincrement,
    title varchar(512) not null unique,
    author varchar(2048) not null,
    description varchar(32768) not null,
    copies int not null,
    available int not null
);

create table lms_borrow (
    uid int not null,
    bid int not null,
    primary key (uid, bid),
    foreign key (uid) references lms_user (uid),
    foreign key (bid) references lms_book (bid)
);
 */

#[inline]
pub fn user_register(req: RequestUserRegister) -> ResponseUserRegister {
    info!("user_register IN {:?}", req);
    let db = database();
    if let Err(err) = db.execute(
        "INSERT INTO lms_user (username, email) VALUES (?1, ?2)",
        [&req.username, &req.email],
    ) {
        info!("user_register ERR {:?}", err);
        ResponseUserRegister {
            success: false,
            uid: 0,
            message: format!("{}", err),
        }
    } else {
        let uid = db.query_row(
            "SELECT uid FROM lms_user WHERE username = ?1",
            [&req.username],
            |row| row.get(0),
        ).unwrap();
        info!("user_register OUT {}", uid);
        ResponseUserRegister {
            success: true,
            uid,
            message: "success".to_string(),
        }
    }
}

#[inline]
pub fn user_name_lookup(req: RequestUserNameLookup) -> ResponseUserNameLookup {
    info!("user_name_lookup IN {:?}", req);
    let db = database();
    let uid = db.query_row(
        "SELECT uid FROM lms_user WHERE username = ?1",
        [&req.username],
        |row| row.get(0),
    );
    match uid {
        Ok(uid) => {
            info!("user_name_lookup OUT {:?}", uid);
            ResponseUserNameLookup {
                success: true,
                uid,
                message: "success".to_string(),
            }
        }
        Err(err) => {
            info!("user_name_lookup ERR {:?}", err);
            ResponseUserNameLookup {
                success: false,
                uid: 0,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_email_lookup(req: RequestUserEmailLookup) -> ResponseUserEmailLookup {
    info!("user_email_lookup IN {:?}", req);
    let uid = database().query_row(
        "SELECT uid FROM lms_user WHERE email = ?1",
        [&req.email],
        |row| row.get(0),
    );
    match uid {
        Ok(uid) => {
            info!("user_email_lookup OUT {:?}", uid);
            ResponseUserEmailLookup {
                success: true,
                uid,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_email_lookup ERR {:?}", err);
            ResponseUserEmailLookup {
                success: false,
                uid: 0,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_alter_name(req: RequestUserAlterName) -> ResponseUserAlterName {
    info!("user_alter_name IN {:?}", req);
    let res = database().execute(
        "UPDATE lms_user SET username = ?1 WHERE uid = ?2",
        [&req.new_username, &req.uid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_alter_name OUT {:?}", req);
            ResponseUserAlterName {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_alter_name ERR {:?}", err);
            ResponseUserAlterName {
                success: false,
                message: format!("{}", err),
            }
        }
    }
}

#[inline]
pub fn user_alter_email(req: RequestUserAlterEmail) -> ResponseUserAlterEmail {
    info!("user_alter_email IN {:?}", req);
    let res = database().execute(
        "UPDATE lms_user SET email = ?1 WHERE uid = ?2",
        [&req.new_email, &req.uid.to_string()],
    );
    match res {
        Ok(_) => {
            info!("user_alter_email OUT {:?}", req);
            ResponseUserAlterEmail {
                success: true,
                message: "success".to_string(),
            }
        },
        Err(err) => {
            info!("user_alter_email ERR {:?}", err);
            ResponseUserAlterEmail {
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
    // select bid from borrow where uid = ?1
    let mut stmt = db
        .prepare("SELECT bid FROM lms_borrow WHERE uid = ?1")
        .unwrap();
    let bids = stmt
        .query_map([&req.uid.to_string()], |row| {
            Ok(row.get(0).unwrap())
        });
    let bids = match bids {
        Ok(bids) => bids,
        Err(err) => {
            info!("user_borrowed ERR {:?}", err);
            return ResponseUserBorrowed {
                success: false,
                bids: "".to_string(),
                message: format!("{}", err),
            };
        }
    };
    let bids = bids
        .map(|x| x.unwrap())
        .collect::<Vec<u64>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    info!("user_borrowed OUT {:?}", bids);
    ResponseUserBorrowed {
        success: true,
        bids: bids.join(","),
        message: "success".to_string(),
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
        "INSERT INTO lms_borrow (uid, bid) VALUES (?1, ?2)",
        [&req.uid.to_string(), &req.bid.to_string()],
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
        "DELETE FROM lms_borrow WHERE uid = ?1 AND bid = ?2",
        [&req.uid.to_string(), &req.bid.to_string()],
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