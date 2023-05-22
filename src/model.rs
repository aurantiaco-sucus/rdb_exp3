use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserRegister {
    pub username: String,
    pub email: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserRegister {
    pub success: bool,
    pub uid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserLookup {
    pub phrase: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserLookup {
    pub success: bool,
    pub uid: u64,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserInfo {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserInfo {
    pub success: bool,
    pub message: String,
    pub username: String,
    pub email: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserAlter {
    pub uid: u64,
    pub username: String,
    pub email: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserAlter {
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
    pub iid_list: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestUserReserved {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseUserReserved {
    pub success: bool,
    pub message: String,
    pub iid_list: String,
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
    pub bid_list: String,
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
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookInstance {
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookInstance {
    pub success: bool,
    pub message: String,
    pub iid_list: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookBorrow {
    pub uid: u64,
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookBorrow {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookReserve {
    pub uid: u64,
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookReserve {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookReturn {
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookReturn {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAdd {
    pub title: String,
    pub author: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAdd {
    pub success: bool,
    pub message: String,
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookRemove {
    pub bid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookRemove {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAlter {
    pub bid: u64,
    pub title: String,
    pub author: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAlter {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookAddInstance {
    pub bid: u64,
    pub status: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookAddInstance {
    pub success: bool,
    pub message: String,
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookRemoveInstance {
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookRemoveInstance {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestBookInstanceInfo {
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseBookInstanceInfo {
    pub success: bool,
    pub message: String,
    pub bid: u64,
    pub status: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestInstanceOccupy {
    pub iid: u64,
    pub status: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseInstanceOccupy {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RequestInstanceRelease {
    pub iid: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ResponseInstanceRelease {
    pub success: bool,
    pub message: String,
}