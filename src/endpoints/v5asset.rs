pub enum Asset {
    GetCoinExchangeRecords,
    GetOptionDeliveryRecord,
    GetUsdcContractSettlement,
    GetSpotAssetInfo,
    GetAllCoinsBalance,
    GetSingleCoinBalance,
    GetTransferableCoin,
    CreateInternalTransfer,
    GetInternalTransferRecords,
    GetSubUid,
    EnableUtForSubUid,
    CreateUniversalTransfer,
    GetUniversalTransferRecords,
    GetAllowedDepositCoinInfo,
    SetDepositAccount,
    GetDepositRecords,
    GetSubAccountDepositRecords,
    GetInternalDepositRecords,
    GetMasterDepositAddress,
    GetSubDepositAddress,
    GetCoinInfo,
    GetWithdrawalRecords,
    GetWithdrawableAmount,
    Withdraw,
    CancelWithdrawal,
}

impl std::fmt::Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Asset::GetCoinExchangeRecords => write!(f, "/v5/asset/exchange/order-record"),
            Asset::GetOptionDeliveryRecord => write!(f, "/v5/asset/delivery-record"),
            Asset::GetUsdcContractSettlement => write!(f, "/v5/asset/settlement-record"),
            Asset::GetSpotAssetInfo => write!(f, "/v5/asset/transfer/query-asset-info"),
            Asset::GetAllCoinsBalance => {
                write!(f, "/v5/asset/transfer/query-account-coins-balance")
            }
            Asset::GetSingleCoinBalance => {
                write!(f, "/v5/asset/transfer/query-account-coin-balance")
            }
            Asset::GetTransferableCoin => write!(f, "/v5/asset/transfer/query-transfer-coin-list"),
            Asset::CreateInternalTransfer => write!(f, "/v5/asset/transfer/inter-transfer"),
            Asset::GetInternalTransferRecords => {
                write!(f, "/v5/asset/transfer/query-inter-transfer-list")
            }
            Asset::GetSubUid => write!(f, "/v5/asset/transfer/query-sub-member-list"),
            Asset::EnableUtForSubUid => write!(f, "/v5/asset/transfer/save-transfer-sub-member"),
            Asset::CreateUniversalTransfer => write!(f, "/v5/asset/transfer/universal-transfer"),
            Asset::GetUniversalTransferRecords => {
                write!(f, "/v5/asset/transfer/query-universal-transfer-list")
            }
            Asset::GetAllowedDepositCoinInfo => write!(f, "/v5/asset/deposit/query-allowed-list"),
            Asset::SetDepositAccount => write!(f, "/v5/asset/deposit/deposit-to-account"),
            Asset::GetDepositRecords => write!(f, "/v5/asset/deposit/query-record"),
            Asset::GetSubAccountDepositRecords => {
                write!(f, "/v5/asset/deposit/query-sub-member-record")
            }
            Asset::GetInternalDepositRecords => {
                write!(f, "/v5/asset/deposit/query-internal-record")
            }
            Asset::GetMasterDepositAddress => write!(f, "/v5/asset/deposit/query-address"),
            Asset::GetSubDepositAddress => write!(f, "/v5/asset/deposit/query-sub-member-address"),
            Asset::GetCoinInfo => write!(f, "/v5/asset/coin/query-info"),
            Asset::GetWithdrawalRecords => write!(f, "/v5/asset/withdraw/query-record"),
            Asset::GetWithdrawableAmount => write!(f, "/v5/asset/withdraw/withdrawable-amount"),
            Asset::Withdraw => write!(f, "/v5/asset/withdraw/create"),
            Asset::CancelWithdrawal => write!(f, "/v5/asset/withdraw/cancel"),
        }
    }
}
