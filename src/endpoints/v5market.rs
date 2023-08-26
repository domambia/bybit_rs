pub enum MarketEnum {
    GetKline,
    GetMarkPriceKline,
    GetIndexPriceKline,
    GetPremiumIndexPriceKline,
    GetInstrumentsInfo,
    GetOrderbook,
    GetTickers,
    GetFundingRateHistory,
    GetPublicTradingHistory,
    GetOpenInterest,
    GetHistoricalVolatility,
    GetInsurance,
    GetRiskLimit,
    GetOptionDeliveryPrice,
}

impl std::fmt::Display for MarketEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MarketEnum::GetKline => write!(f, "/v5/market/kline"),
            MarketEnum::GetMarkPriceKline => write!(f, "/v5/market/mark-price-kline"),
            MarketEnum::GetIndexPriceKline => write!(f, "/v5/market/index-price-kline"),
            MarketEnum::GetPremiumIndexPriceKline => {
                write!(f, "/v5/market/premium-index-price-kline")
            }
            MarketEnum::GetInstrumentsInfo => write!(f, "/v5/market/instruments-info"),
            MarketEnum::GetOrderbook => write!(f, "/v5/market/orderbook"),
            MarketEnum::GetTickers => write!(f, "/v5/market/tickers"),
            MarketEnum::GetFundingRateHistory => write!(f, "/v5/market/funding/history"),
            MarketEnum::GetPublicTradingHistory => write!(f, "/v5/market/recent-trade"),
            MarketEnum::GetOpenInterest => write!(f, "/v5/market/open-interest"),
            MarketEnum::GetHistoricalVolatility => write!(f, "/v5/market/historical-volatility"),
            MarketEnum::GetInstrumentsInfo => write!(f, "/v5/market/insurance"),
            MarketEnum::GetRiskLimit => write!(f, "/v5/market/risk-limit"),
            MarketEnum::GetOptionDeliveryPrice => write!(f, "/v5/market/delivery-price"),
            MarketEnum::GetInsurance => todo!(),
        }
    }
}
