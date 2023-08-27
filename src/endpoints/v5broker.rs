pub enum Broker {
    GetBrokerEarnings,
}

impl std::fmt::Display for Broker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Broker::GetBrokerEarnings => write!(f, "/v5/broker/earning-record"),
        }
    }
}
