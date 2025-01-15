use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedDeal {
    /// Deal identifier.
    pub deal_id: String,
    /// Deal status.
    pub status: crate::rest_models::AffectedDealStatus,
}

/// Affected deal status.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AffectedDealStatus {
    /// Amended.
    Amended,
    /// Deleted.
    Deleted,
    /// Fully closed.
    FullyClosed,
    /// Opened.
    Opened,
    /// Partially closed.
    PartiallyClosed,
}

/// Deal status.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealStatus {
    /// Accepted.
    Accepted,
    /// Rejected.
    Rejected,
}

/// Deal direction.
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    /// Buy.
    #[default]
    Buy,
    /// Sell.
    Sell,
}

/// Describes the error (or success) condition for the specified trading operation.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DealReason {
    /// The account is not enabled to trade.
    AccountNotEnabledToTrading,
    /// The level of the attached stop or limit is not valid.
    AttachedOrderLevelError,
    /// The trailing stop value is invalid.
    AttachedOrderTrailingStopError,
    /// Cannot change the stop type.
    CannotChangeStopType,
    /// Cannot remove the stop.
    CannotRemoveStop,
    /// We are not taking opening deals on a Controlled Risk basis on this market.
    ClosingOnlyTradesAcceptedOnThisMarket,
    /// You are currently restricted from opening any new positions on your account.
    ClosingsOnlyAccount,
    /// Resubmitted request does not match the original order.
    ConflictingOrder,
    /// Instrument has an error - check the order's currency is the instrument's currency
    /// (see the market's details); otherwise please contact support.
    ContactSupportInstrumentError,
    /// Sorry we are unable to process this order. The stop or limit level you have requestedi
    /// is not a valid trading level in the underlying market.
    CrSpacing,
    /// The order has been rejected as it is a duplicate of a previously issued order.
    DuplicateOrderError,
    /// Exchange check failed. Please call in for assistance.
    ExchangeManualOverride,
    /// Order expiry is less than the sprint market's minimum expiry. Check the sprint market's
    /// market details for the allowable expiries.
    ExpiryLessThanSprintMarketMinExpiry,
    /// The total size of deals placed on this market in a short period has exceeded our limits.
    /// Please wait before attempting to open further positions on this market.
    FinanceRepeatDealing,
    /// Ability to force open in different currencies on same market not allowed.
    ForceOpenOnSameMarketDifferentCurrency,
    /// An error has occurred but no detailed information is available. Check transaction
    /// history or contact support for further information.
    GeneralError,
    /// The working order has been set to expire on a past date.
    GoodTillDateInThePast,
    /// The requested market was not found.
    InstrumentNotFound,
    /// Instrument not tradeable in this currency.
    InstrumentNotTradeableInThisCurrency,
    /// The account has not enough funds available for the requested trade.
    InsufficientFunds,
    /// The market level has moved and has been rejected.
    LevelToleranceError,
    /// The deal has been rejected because the limit level is inconsistent with current market
    /// price given the direction.
    LimitOrderWrongSideOfMarket,
    /// The manual order timeout limit has been reached.
    ManualOrderTimeout,
    /// Order declined during margin checks Check available funds.
    MarginError,
    /// The market is currently closed.
    MarketClosed,
    /// The market is currently closed with edits.
    MarketClosedWithEdits,
    /// The epic is due to expire shortly, client should deal in the next available contract.
    MarketClosing,
    /// The market does not allow opening shorting positions.
    MarketNotBorrowable,
    /// The market is currently offline.
    MarketOffline,
    /// The epic does not support 'Market' order type.
    MarketOrdersNotAllowedOnInstrument,
    /// The market can only be traded over the phone.
    MarketPhoneOnly,
    /// The market has been rolled to the next period.
    MarketRolled,
    /// The requested market is not allowed to this account.
    MarketUnavailableToClient,
    /// The order size exceeds the instrument's maximum configured value for auto-hedging
    /// the exposure of a deal.
    MaxAutoSizeExceeded,
    /// The order size is too small.
    MinimumOrderSizeError,
    /// The limit level you have requested is closer to the market level than the existing
    /// stop. When the market is closed you can only move the limit order further away from
    /// the current market level.
    MoveAwayOnlyLimit,
    /// The stop level you have requested is closer to the market level than the existing
    /// stop level. When the market is closed you can only move the stop level further away
    /// from the current market level.
    MoveAwayOnlyStop,
    /// The order level you have requested is moving closer to the market level than the
    /// exisiting order level. When the market is closed you can only move the order further
    /// away from the current market level.
    MoveAwayOnlyTriggerLevel,
    /// You are not permitted to open a non-controlled risk position on this account.
    NcrPositionsOnCrAccount,
    /// Opening CR position in opposite direction to existing CR position not allowed.
    OpposingDirectionOrdersNotAllowed,
    /// The deal has been rejected to avoid having long and short open positions on the same
    /// market or having long and short open positions and working orders on the same epic.
    OpposingPositionsNotAllowed,
    /// Order declined; please contact Support.
    OrderDeclined,
    /// The order is locked and cannot be edited by the user.
    OrderLocked,
    /// The order has not been found.
    OrderNotFound,
    /// The order size cannot be filled at this price at the moment.
    OrderSizeCannotBeFilled,
    /// The total position size at this stop level is greater than the size allowed on this market. Please reduce the size of the order.
    OverNormalMarketSize,
    /// Position cannot be deleted as it has been partially closed.
    PartialyClosedPositionNotDeleted,
    /// The deal has been rejected because of an existing position. Either set the 'force open' to be true or cancel opposing position.
    PositionAlreadyExistsInOppositeDirection,
    /// Position cannot be cancelled. Check transaction history or contact support for further information.
    PositionNotAvailableToCancel,
    /// Cannot close this position. Either the position no longer exists, or the size available to close is less than the size specified.
    PositionNotAvailableToClose,
    /// The position has not been found.
    PositionNotFound,
    /// Invalid attempt to submit a CFD trade on a spreadbet account.
    RejectCfdOrderOnSpreadbetAccount,
    /// Invalid attempt to submit a spreadbet trade on a CFD account.
    RejectSpreadbetOrderOnCfdAccount,
    /// Order size is not an increment of the value specified for the market.
    SizeIncrement,
    /// The expiry of the position would have fallen after the closing time of the market.
    SprintMarketExpiryAfterMarketClose,
    /// The market does not allow stop or limit attached orders.
    StopOrLimitNotAllowed,
    /// The order requires a stop.
    StopRequiredError,
    /// The submitted strike level is invalid.
    StrikeLevelTolerance,
    /// The operation completed successfully.
    Success,
    /// The market or the account do not allow for trailing stops.
    TrailingStopNotAllowed,
    /// The operation resulted in an unknown result condition. Check transaction history or contact support for further information.
    Unknown,
    /// The requested operation has been attempted on the wrong direction.
    WrongSideOfMarket,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionStatus {
    Amended,
    Closed,
    Deleted,
    Open,
    PartiallyClosed,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Confirm {
    /// Affected deals.
    pub affected_deals: Vec<AffectedDeal>,
    /// Transaction date.
    pub date: String,
    /// Deal identifier.
    pub deal_id: String,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal status.
    pub deal_status: DealStatus,
    /// Deal direction.
    pub direction: Direction,
    /// Instrument epic identifier.
    pub epic: String,
    /// Instrument expiry.
    pub expiry: Option<String>,
    /// True if guaranteed stop.
    pub guaranteed_stop: bool,
    /// Level at which the deal was opened.
    pub level: Option<f64>,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Profit.
    pub profit: Option<f64>,
    /// Profit currency.
    pub profit_currency: Option<String>,
    /// Describes the error (or success) condition for the specified trading operation.
    pub reason: DealReason,
    /// Size of the deal.
    pub size: Option<f64>,
    /// Position status.
    pub status: Option<PositionStatus>,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// True if trailing stop.
    pub trailing_stop: bool,
}

use chrono::{DateTime, Utc};

/// Open Position Updates for an account (OPU)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Opu {
    /// Deal direction.
    pub direction: Direction,
    /// Limit level.
    pub limit_level: Option<f64>,
    /// Deal identifier.
    pub deal_id: String,
    /// Originating deal identifier.
    pub deal_id_origin: Option<String>,
    /// Stop level.
    pub stop_level: Option<f64>,
    /// Instrument expiry.
    pub expiry: Option<String>,
    /// Event date and time.
    pub timestamp: DateTime<Utc>,
    /// Trade size.
    pub size: f64,
    /// Position status.
    pub status: PositionStatus,
    /// Instrument EPIC identifier.
    pub epic: String,
    /// Trade level.
    pub level: Option<f64>,
    /// True if a guaranteed stop is in place.
    pub guaranteed_stop: bool,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal status.
    pub deal_status: DealStatus,
    /// User channel.
    pub channel: Option<String>,
    /// Currency.
    pub currency: Option<String>,
}

/// Working Order Updates for an account (WOU)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Wou {
    /// Deal direction.
    pub direction: Direction,
    /// Limit distance.
    pub limit_distance: Option<f64>,
    /// Deal identifier.
    pub deal_id: String,
    /// Stop distance.
    pub stop_distance: Option<f64>,
    /// Instrument expiry.
    pub expiry: Option<String>,
    /// Event date and time.
    pub timestamp: DateTime<Utc>,
    /// Trade size.
    pub size: f64,
    /// Working order status.
    pub status: WouStatus,
    /// Instrument EPIC identifier.
    pub epic: String,
    /// Trade level.
    pub level: Option<f64>,
    /// True if a guaranteed stop is in place.
    pub guaranteed_stop: bool,
    /// Deal reference.
    pub deal_reference: String,
    /// Deal status.
    pub deal_status: DealStatus,
    /// Currency.
    pub currency: Option<String>,
    /// Order type.
    pub order_type: OrderType,
    /// Time in force.
    pub time_in_force: TimeInForce,
    /// Good until specified date.
    pub good_till_date: Option<DateTime<Utc>>,
    /// User channel.
    pub channel: Option<String>,
}

/// Working Order Status
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WouStatus {
    Open,
    Updated,
    Deleted,
}

/// Order Type
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Stop,
}

/// Time in Force
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    GoodTillCancelled,
    GoodTillDate,
}