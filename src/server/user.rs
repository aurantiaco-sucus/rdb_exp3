use log::info;
use crate::model::*;

pub fn user_register(req: RequestUserRegister) -> ResponseUserRegister {
    ResponseUserRegister {
        success: true,
        uid: 0,
        message: "success".to_string(),
    }
}

pub fn user_name_lookup(req: RequestUserNameLookup) -> ResponseUserNameLookup {
    info!("user_name_lookup: {:?}", req);
    ResponseUserNameLookup {
        success: true,
        uid: 0,
        message: "success".to_string(),
    }
}

pub fn user_email_lookup(req: RequestUserEmailLookup) -> ResponseUserEmailLookup {
    ResponseUserEmailLookup {
        success: true,
        uid: 0,
        message: "success".to_string(),
    }
}

pub fn user_alter