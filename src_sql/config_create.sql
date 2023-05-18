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