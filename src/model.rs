use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserRegister {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserRegister {
    pub success: bool,
    pub uid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserNameLookup {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserNameLookup {
    pub success: bool,
    pub uid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserEmailLookup {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserEmailLookup {
    pub success: bool,
    pub uid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserAlterName {
    pub uid: u64,
    pub new_username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserAlterName {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserAlterPassword {
    pub uid: u64,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserAlterPassword {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserAlterEmail {
    pub uid: u64,
    pub new_email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserAlterEmail {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserBorrowed {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserBorrowed {
    pub success: bool,
    pub message: String,
    pub bids: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserUnregister {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserUnregister {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookSearch {
    pub phrase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookSearch {
    pub success: bool,
    pub message: String,
    pub bids: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookInfo {
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookInfo {
    pub success: bool,
    pub message: String, 
    pub title: String,
    pub author: String,
    pub description: String,
    pub copies: u64,
    pub available: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookBorrow {
    pub uid: u64,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookBorrow {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookReturn {
    pub uid: u64,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookReturn {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookRenew {
    pub uid: u64,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookRenew {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestAdminRegister {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseAdminRegister {
    pub success: bool,
    pub aid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAdd {
    pub aid: u64,
    pub title: String,
    pub author: String,
    pub description: String,
    pub copies: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAdd {
    pub success: bool,
    pub message: String,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookRemove {
    pub aid: u64,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookRemove {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAlter {
    pub aid: u64,
    pub bid: u64,
    pub title: String,
    pub author: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAlter {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAlterCopies {
    pub aid: u64,
    pub bid: u64,
    pub copies: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAlterCopies {
    pub success: bool,
    pub message: String,
}