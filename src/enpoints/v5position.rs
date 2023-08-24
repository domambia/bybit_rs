enum Position {
    GET_POSITIONS,
    SET_LEVERAGE,
    SWITCH_MARGIN_MODE,
    SET_TP_SL_MODE,
    SWITCH_POSITION_MODE,
    SET_RISK_LIMIT,
    SET_TRADING_STOP,
    SET_AUTO_ADD_MARGIN,
    GET_EXECUTIONS,
    GET_CLOSED_PNL,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position::GET_POSITIONS => write!(f, "/v5/position/list"),
            Position::SET_LEVERAGE => write!(f, "/v5/position/set-leverage"),
            Position::SWITCH_MARGIN_MODE => write!(f, "/v5/position/switch-isolated"),
            Position::SET_TP_SL_MODE => write!(f, "/v5/position/set-tpsl-mode"),
            Position::SWITCH_POSITION_MODE => write!(f, "/v5/position/switch-mode"),
            Position::SET_RISK_LIMIT => write!(f, "/v5/position/set-risk-limit"),
            Position::SET_TRADING_STOP => write!(f, "/v5/position/trading-stop"),
            Position::SET_AUTO_ADD_MARGIN => write!(f, "/v5/position/set-auto-add-margin"),
            Position::GET_EXECUTIONS => write!(f, "/v5/execution/list"),
            Position::GET_CLOSED_PNL => write!(f, "/v5/position/closed-pnl"),
        }
    }
}
