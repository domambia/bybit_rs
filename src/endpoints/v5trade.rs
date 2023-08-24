pub enum Trade {
    PlaceOrder,
    AmendOrder,
    CancelOrder,
    GetOpenOrders,
    CancelAllOrders,
    GetOrderHistory,
    BatchPlaceOrder,
    BatchAmendOrder,
    BatchCancelOrder,
    GetBorrowQuota,
    SetDcp,
}

impl std::fmt::Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Trade::PlaceOrder => write!(f, "/v5/order/create"),
            Trade::AmendOrder => write!(f, "/v5/order/amend"),
            Trade::CancelOrder => write!(f, "/v5/order/cancel"),
            Trade::GetOpenOrders => write!(f, "/v5/order/realtime"),
            Trade::CancelAllOrders => write!(f, "/v5/order/cancel-all"),
            Trade::GetOrderHistory => write!(f, "/v5/order/history"),
            Trade::BatchPlaceOrder => write!(f, "/v5/order/create-batch"),
            Trade::BatchAmendOrder => write!(f, "/v5/order/amend-batch"),
            Trade::BatchCancelOrder => write!(f, "/v5/order/cancel-batch"),
            Trade::GetBorrowQuota => write!(f, "/v5/order/spot-borrow-check"),
            Trade::SetDcp => write!(f, "/v5/order/disconnected-cancel-all"),
        }
    }
}
