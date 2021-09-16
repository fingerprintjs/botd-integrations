/// An error that occurred during bot detection
pub enum BotdError {
    /// A regex syntax error.
    RegexSyntax(String),
    /// Passed HTML string doesn't contain <head> tag
    WrongHTML,
    /// Can't extract botd token.
    NoTokenInConfig,
    /// Passed HTML string doesn't contain <head> tag
    Disabled,
    /// Can't extract botd request id from headers.
    NoRequestIdInHeaders,
    /// Can't extract botd request status from headers.
    NoRequestStatusInHeaders,
    /// Can't extract botd error descriptions from headers.
    NoErrorDescriptionInHeaders,
    /// Can't cast to string.
    ToStringCast(String),
    /// Error during request sending.
    SendError(String),
    /// Can't extract botd request status from headers.
    NoRequestIdInCookie,
}

impl ToString for BotdError {
    fn to_string(&self) -> String {
        match self {
            BotdError::RegexSyntax(re) => format!("Can't create regex {}", re),
            BotdError::WrongHTML => String::from("Can't find head tag in response body"),
            BotdError::NoTokenInConfig => String::from("Can't get botd token from config"),
            BotdError::Disabled => String::from("Bot detection disabled"),
            BotdError::NoRequestIdInHeaders => String::from("Request id cannot be found in headers"),
            BotdError::NoRequestStatusInHeaders => String::from("Request status cannot be found in headers"),
            BotdError::NoErrorDescriptionInHeaders => String::from("Request status is not processed, but error description cannot be found."),
            BotdError::ToStringCast(name) => format!("Can't cast {} to string", name),
            BotdError::SendError(desc) => format!("Error occurred during sending to {} backend", desc),
            BotdError::NoRequestIdInCookie => String::from("Request id cannot be found in cookie"),
        }
    }
}