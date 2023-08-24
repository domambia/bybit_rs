pub enum User {
    CreateSubUid,
    CreateSubApiKey,
    GetSubUidList,
    FreezeSubUid,
    GetApiKeyInformation,
    ModifyMasterApiKey,
    ModifySubApiKey,
    DeleteMasterApiKey,
    DeleteSubApiKey,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            User::CreateSubUid => write!(f, "/v5/user/create-sub-member"),
            User::CreateSubApiKey => write!(f, "/v5/user/create-sub-api"),
            User::GetSubUidList => write!(f, "/v5/user/query-sub-members"),
            User::FreezeSubUid => write!(f, "/v5/user/frozen-sub-member"),
            User::GetApiKeyInformation => write!(f, "/v5/user/query-api"),
            User::ModifyMasterApiKey => write!(f, "/v5/user/update-api"),
            User::ModifySubApiKey => write!(f, "/v5/user/update-sub-api"),
            User::DeleteMasterApiKey => write!(f, "/v5/user/delete-api"),
            User::DeleteSubApiKey => write!(f, "/v5/user/delete-sub-api"),
        }
    }
}
