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

create table lms_history (
    uid integer not null,
    iid integer not null,
    date text not null,
    return_date text not null,
    foreign key (uid) references lms_user (uid),
    foreign key (iid) references lms_instance (iid)
);

create index lms_history_uid on lms_history (uid);
create index lms_history_iid on lms_history (iid);
create index lms_history_date on lms_history (date);
create index lms_history_return_date on lms_history (return_date);

create trigger lms_occupation_remove
    after delete on lms_occupation
    when old.kind = 0
    begin
        insert into lms_history (uid, iid, date, return_date)
        values (old.uid, old.iid, old.date, date('now'));
    end;