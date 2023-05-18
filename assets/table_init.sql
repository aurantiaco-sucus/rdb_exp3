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

/* trigger to update available copies of a book and deny borrow if no copies are available */
create trigger lms_borrow_trigger
    before insert on lms_borrow
    for each row
    when (select available from lms_book where bid = new.bid) = 0
    begin
        select raise(abort, 'No copies of this book are available');
    end;

/* trigger to update available copies of a book when a book is returned */
create trigger lms_return_trigger
    after delete on lms_borrow
    for each row
    begin
        update lms_book set available = available + 1 where bid = old.bid;
    end;