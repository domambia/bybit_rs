pub enum Market {
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

impl std::fmt::Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Market::GetKline => write!(f, "/v5/market/kline"),
            Market::GetMarkPriceKline => write!(f, "/v5/market/mark-price-kline"),
            Market::GetIndexPriceKline => write!(f, "/v5/market/index-price-kline"),
            Market::GetPremiumIndexPriceKline => {
                write!(f, "/v5/market/premium-index-price-kline")
            }
            Market::GetInstrumentsInfo => write!(f, "/v5/market/instruments-info"),
            Market::GetOrderbook => write!(f, "/v5/market/orderbook"),
            Market::GetTickers => write!(f, "/v5/market/tickers"),
            Market::GetFundingRateHistory => write!(f, "/v5/market/funding/history"),
            Market::GetPublicTradingHistory => write!(f, "/v5/market/recent-trade"),
            Market::GetOpenInterest => write!(f, "/v5/market/open-interest"),
            Market::GetHistoricalVolatility => write!(f, "/v5/market/historical-volatility"),
            Market::GetInstrumentsInfo => write!(f, "/v5/market/insurance"),
            Market::GetRiskLimit => write!(f, "/v5/market/risk-limit"),
            Market::GetOptionDeliveryPrice => write!(f, "/v5/market/delivery-price"),
            Market::GetInsurance => todo!(),
        }
    }
}
