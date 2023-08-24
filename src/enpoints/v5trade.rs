enum Trade {
    PLACE_ORDER,
    AMEND_ORDER,
    CANCEL_ORDER,
    GET_OPEN_ORDERS,
    CANCEL_ALL_ORDERS,
    GET_ORDER_HISTORY,
    BATCH_PLACE_ORDER,
    BATCH_AMEND_ORDER,
    BATCH_CANCEL_ORDER,
    GET_BORROW_QUOTA,
    SET_DCP,
}

impl std::fmt::Display for Trade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Trade::PLACE_ORDER => write!(f, "/v5/order/create"),
            Trade::AMEND_ORDER => write!(f, "/v5/order/amend"),
            Trade::CANCEL_ORDER => write!(f, "/v5/order/cancel"),
            Trade::GET_OPEN_ORDERS => write!(f, "/v5/order/realtime"),
            Trade::CANCEL_ALL_ORDERS => write!(f, "/v5/order/cancel-all"),
            Trade::GET_ORDER_HISTORY => write!(f, "/v5/order/history"),
            Trade::BATCH_PLACE_ORDER => write!(f, "/v5/order/create-batch"),
            Trade::BATCH_AMEND_ORDER => write!(f, "/v5/order/amend-batch"),
            Trade::BATCH_CANCEL_ORDER => write!(f, "/v5/order/cancel-batch"),
            Trade::GET_BORROW_QUOTA => write!(f, "/v5/order/spot-borrow-check"),
            Trade::SET_DCP => write!(f, "/v5/order/disconnected-cancel-all"),
        }
    }
}
