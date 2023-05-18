use log::info;
use crate::model::*;
use crate::server::database;

/*
In case you forget about it, here is how the tables look like:

create table lms_user (
    uid int primary key,
    username varchar(512) not null unique,
    email varchar(512) not null unique
);

create table lms_book (
    bid int primary key,
    title varchar(512) not null unique,
    author varchar(2048) not null,
    description varchar(32768) not null,
    copies int not null,
    available int not null
);

create table lms_borrow (
    uid int not null,
    bid int not null,
    borrow_date date not null,
    return_date date not null,
    primary key (uid, bid),
    foreign key (uid) references lms_user (uid),
    foreign key (bid) references lms_book (bid)
);
 */

#[inline]
pub fn user_register(req: RequestUserRegister) -> ResponseUserRegister {
    info!("user_register: {:?}", req);
    if let Err(err) = database().execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        [&req.username, &req.email],
    ) {
        info!("user_register<failed>: {}", err);
        ResponseUserRegister {
            success: false,
            uid: 0,
            message: format!("{}", err),
        }
    } else {
        let uid = database().query_row(
            "SELECT uid FROM users WHERE name = ?1",
            [&req.username],
            |row| row.get(0),
        ).unwrap();
        info!("user_register<success>: {}", uid);
        ResponseUserRegister {
            success: true,
            uid,
            message: String::new(),
        }
    }
}

#[inline]
pub fn user_name_lookup(req: RequestUserNameLookup) -> ResponseUserNameLookup {
    info!("user_name_lookup: {:?}", req);
    let uid = database().query_row(
        "SELECT uid FROM users WHERE name = ?1",
        [&req.username],
        |row| row.get(0),
    );
    match uid {
        Ok(uid) => ResponseUserNameLookup {
            success: true,
            uid,
            message: "success".to_string(),
        },
        Err(err) => ResponseUserNameLookup {
            success: false,
            uid: 0,
            message: format!("{}", err),
        }
    }
}

#[inline]
pub fn user_email_lookup(req: RequestUserEmailLookup) -> ResponseUserEmailLookup {
    info!("user_email_lookup: {:?}", req);
    ResponseUserEmailLookup {
        success: true,
        uid: 0,
        message: "success".to_string(),
    }
}

#[inline]
pub fn user_alter_name(req: RequestUserAlterName) -> ResponseUserAlterName {
    info!("user_alter_name: {:?}", req);
    ResponseUserAlterName {
        success: true,
        message: "success".to_string(),
    }
}

#[inline]
pub fn user_alter_email(req: RequestUserAlterEmail) -> ResponseUserAlterEmail {
    info!("user_alter_email: {:?}", req);
    ResponseUserAlterEmail {
        success: true,
        message: "success".to_string(),
    }
}

#[inline]
pub fn user_borrowed(req: RequestUserBorrowed) -> ResponseUserBorrowed {
    info!("user_borrowed: {:?}", req);
    ResponseUserBorrowed {
        success: true,
        message: "success".to_string(),
        bids: "".to_string(),
    }
}

#[inline]
pub fn user_unregister(req: RequestUserUnregister) -> ResponseUserUnregister {
    info!("user_unregister: {:?}", req);
    ResponseUserUnregister {
        success: true,
        message: "success".to_string(),
    }
}

#[inline]
pub fn user_borrow(req: RequestBookBorrow) -> ResponseBookBorrow {
    ResponseBookBorrow {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn user_return(req: RequestBookReturn) -> ResponseBookReturn {
    ResponseBookReturn {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn user_renew(req: RequestBookRenew) -> ResponseBookRenew {
    ResponseBookRenew {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn admin_add(req: RequestBookAdd) -> ResponseBookAdd {
    ResponseBookAdd {
        success: false,
        message: "Not implemented".to_string(),
        bid: 0,
    }
}

#[inline]
pub fn admin_remove(req: RequestBookRemove) -> ResponseBookRemove {
    ResponseBookRemove {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn admin_alter(req: RequestBookAlter) -> ResponseBookAlter {
    ResponseBookAlter {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn admin_alter_copies(req: RequestBookAlterCopies) -> ResponseBookAlterCopies {
    ResponseBookAlterCopies {
        success: false,
        message: "Not implemented".to_string(),
    }
}

#[inline]
pub fn book_search(req: RequestBookSearch) -> ResponseBookSearch {
    ResponseBookSearch {
        success: false,
        message: "Not implemented".to_string(),
        bids: "".to_string(),
    }
}

#[inline]
pub fn book_info(req: RequestBookInfo) -> ResponseBookInfo {
    ResponseBookInfo {
        success: false,
        message: "Not implemented".to_string(),
        title: "".to_string(),
        author: "".to_string(),
        description: "".to_string(),
        copies: 0,
        available: 0,
    }
}