pub enum SpotLeverageToken {
    GetLeveragedTokenInfo,
    GetLeveragedTokenMarket,
    Purchase,
    Redeem,
    GetPurchaseRedemptionRecords,
}

impl std::fmt::Display for SpotLeverageToken {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpotLeverageToken::GetLeveragedTokenInfo => write!(f, "/v5/spot-lever-token/info"),
            SpotLeverageToken::GetLeveragedTokenMarket => {
                write!(f, "/v5/spot-lever-token/reference")
            }
            SpotLeverageToken::Purchase => write!(f, "/v5/spot-lever-token/purchase"),
            SpotLeverageToken::Redeem => write!(f, "/v5/spot-lever-token/redeem"),
            SpotLeverageToken::GetPurchaseRedemptionRecords => {
                write!(f, "/v5/spot-lever-token/order-record")
            }
        }
    }
}
