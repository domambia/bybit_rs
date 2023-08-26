pub enum SpotMarginTrade {
    // UTA endpoints
    ToggleMarginTrade,
    SetLeverage,
    // normal mode (non-UTA) endpoints
    NormalGetMarginCoinInfo,
    NormalGetBorrowableCoinInfo,
    NormalGetInterestQuota,
    NormalGetLoanAccountInfo,
    NormalBorrow,
    NormalRepay,
    NormalGetBorrowOrderDetail,
    NormalGetRepaymentOrderDetail,
    NormalToggleMarginTrade,
}

impl std::fmt::Display for SpotMarginTrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpotMarginTrade::ToggleMarginTrade => write!(f, "/v5/spot-margin-trade/switch-mode"),
            SpotMarginTrade::SetLeverage => write!(f, "/v5/spot-margin-trade/set-leverage"),
            SpotMarginTrade::NormalGetMarginCoinInfo => {
                write!(f, "/v5/spot-cross-margin-trade/pledge-token")
            }
            SpotMarginTrade::NormalGetBorrowableCoinInfo => {
                write!(f, "/v5/spot-cross-margin-trade/borrow-token")
            }
            SpotMarginTrade::NormalGetInterestQuota => {
                write!(f, "/v5/spot-cross-margin-trade/loan-info")
            }
            SpotMarginTrade::NormalGetLoanAccountInfo => {
                write!(f, "/v5/spot-cross-margin-trade/account")
            }
            SpotMarginTrade::NormalBorrow => write!(f, "/v5/spot-cross-margin-trade/loan"),
            SpotMarginTrade::NormalRepay => write!(f, "/v5/spot-cross-margin-trade/repay"),
            SpotMarginTrade::NormalGetBorrowOrderDetail => {
                write!(f, "/v5/spot-cross-margin-trade/orders")
            }
            SpotMarginTrade::NormalGetRepaymentOrderDetail => {
                write!(f, "/v5/spot-cross-margin-trade/repay-history")
            }
            SpotMarginTrade::NormalToggleMarginTrade => {
                write!(f, "/v5/spot-cross-margin-trade/switch")
            }
        }
    }
}
