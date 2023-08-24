enum Market {
    GET_KLINE,
    GET_MARK_PRICE_KLINE,
    GET_INDEX_PRICE_KLINE,
    GET_PREMIUM_INDEX_PRICE_KLINE,
    GET_INSTRUMENTS_INFO,
    GET_ORDERBOOK,
    GET_TICKERS,
    GET_FUNDING_RATE_HISTORY,
    GET_PUBLIC_TRADING_HISTORY,
    GET_OPEN_INTEREST,
    GET_HISTORICAL_VOLATILITY,
    GET_INSURANCE,
    GET_RISK_LIMIT,
    GET_OPTION_DELIVERY_PRICE,
}

impl std::fmt::Display for Market {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Market::GET_KLINE => write!(f, "/v5/market/kline"),
            Market::GET_MARK_PRICE_KLINE => write!(f, "/v5/market/mark-price-kline"),
            Market::GET_INDEX_PRICE_KLINE => write!(f, "/v5/market/index-price-kline"),
            Market::GET_PREMIUM_INDEX_PRICE_KLINE => {
                write!(f, "/v5/market/premium-index-price-kline")
            }
            Market::GET_INSTRUMENTS_INFO => write!(f, "/v5/market/instruments-info"),
            Market::GET_ORDERBOOK => write!(f, "/v5/market/orderbook"),
            Market::GET_TICKERS => write!(f, "/v5/market/tickers"),
            Market::GET_FUNDING_RATE_HISTORY => write!(f, "/v5/market/funding/history"),
            Market::GET_PUBLIC_TRADING_HISTORY => write!(f, "/v5/market/recent-trade"),
            Market::GET_OPEN_INTEREST => write!(f, "/v5/market/open-interest"),
            Market::GET_HISTORICAL_VOLATILITY => write!(f, "/v5/market/historical-volatility"),
            Market::GET_INSURANCE => write!(f, "/v5/market/insurance"),
            Market::GET_RISK_LIMIT => write!(f, "/v5/market/risk-limit"),
            Market::GET_OPTION_DELIVERY_PRICE => write!(f, "/v5/market/delivery-price"),
        }
    }
}
