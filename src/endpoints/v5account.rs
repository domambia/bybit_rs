pub enum Account {
    GetWalletBalance,
    UpgradeToUnifiedAccount,
    GetBorrowHistory,
    GetCollateralInfo,
    GetCoinGreeks,
    GetFeeRate,
    GetAccountInfo,
    GetTransactionLog,
    SetMarginMode,
    SetMMP,
    ResetMMP,
    GetMMPState,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Account::GetWalletBalance => write!(f, "/v5/account/wallet-balance"),
            Account::UpgradeToUnifiedAccount => write!(f, "/v5/account/upgrade-to-uta"),
            Account::GetBorrowHistory => write!(f, "/v5/account/borrow-history"),
            Account::GetCollateralInfo => write!(f, "/v5/account/collateral-info"),
            Account::GetCoinGreeks => write!(f, "/v5/asset/coin-greeks"),
            Account::GetFeeRate => write!(f, "/v5/account/fee-rate"),
            Account::GetAccountInfo => write!(f, "/v5/account/info"),
            Account::GetTransactionLog => write!(f, "/v5/account/transaction-log"),
            Account::SetMarginMode => write!(f, "/v5/account/set-margin-mode"),
            Account::SetMMP => write!(f, "/v5/account/mmp-modify"),
            Account::ResetMMP => write!(f, "/v5/account/mmp-reset"),
            Account::GetMMPState => write!(f, "/v5/account/mmp-state"),
        }
    }
}
