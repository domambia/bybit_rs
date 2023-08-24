enum User {
    CREATE_SUB_UID,
    CREATE_SUB_API_KEY,
    GET_SUB_UID_LIST,
    FREEZE_SUB_UID,
    GET_API_KEY_INFORMATION,
    MODIFY_MASTER_API_KEY,
    MODIFY_SUB_API_KEY,
    DELETE_MASTER_API_KEY,
    DELETE_SUB_API_KEY,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            User::CREATE_SUB_UID => write!(f, "/v5/user/create-sub-member"),
            User::CREATE_SUB_API_KEY => write!(f, "/v5/user/create-sub-api"),
            User::GET_SUB_UID_LIST => write!(f, "/v5/user/query-sub-members"),
            User::FREEZE_SUB_UID => write!(f, "/v5/user/frozen-sub-member"),
            User::GET_API_KEY_INFORMATION => write!(f, "/v5/user/query-api"),
            User::MODIFY_MASTER_API_KEY => write!(f, "/v5/user/update-api"),
            User::MODIFY_SUB_API_KEY => write!(f, "/v5/user/update-sub-api"),
            User::DELETE_MASTER_API_KEY => write!(f, "/v5/user/delete-api"),
            User::DELETE_SUB_API_KEY => write!(f, "/v5/user/delete-sub-api"),
        }
    }
}
