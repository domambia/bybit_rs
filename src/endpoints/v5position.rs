pub enum Position {
    GetPositions,
    SetLeverage,
    SwitchMarginMode,
    SetTpSlMode,
    SwitchPositionMode,
    SetRiskLimit,
    SetTradingStop,
    SetAutoAddMargin,
    GetExecutions,
    GetClosedPnl,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Position::GetPositions => write!(f, "/v5/position/list"),
            Position::SetLeverage => write!(f, "/v5/position/set-leverage"),
            Position::SwitchMarginMode => write!(f, "/v5/position/switch-isolated"),
            Position::SetTpSlMode => write!(f, "/v5/position/set-tpsl-mode"),
            Position::SwitchPositionMode => write!(f, "/v5/position/switch-mode"),
            Position::SetRiskLimit => write!(f, "/v5/position/set-risk-limit"),
            Position::SetTradingStop => write!(f, "/v5/position/trading-stop"),
            Position::SetAutoAddMargin => write!(f, "/v5/position/set-auto-add-margin"),
            Position::GetExecutions => write!(f, "/v5/execution/list"),
            Position::GetClosedPnl => write!(f, "/v5/position/closed-pnl"),
        }
    }
}
