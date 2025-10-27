

#[derive(Debug, Clone)]
pub enum MatchingError {
    MatchingFailure,
    InvalidRuleApplication,
    UnknownError,
    UnknownRule,
}