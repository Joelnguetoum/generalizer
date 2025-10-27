#[derive(Debug, Clone)]
pub enum ConfigurationError {
    SolveFailed,
    ConstrainedGeneralisationFailed,
    NonGroundTerm,
    InvalidRuleApplication,
    EmptyActiveSet,
    InvalidState,
    RuleApplicationError,
    UnknownRule,
}

impl std::fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigurationError::SolveFailed => write!(f, "Solve-failed"),
            ConfigurationError::ConstrainedGeneralisationFailed => write!(f, "Constrained generalisation failed"),
            ConfigurationError::NonGroundTerm => write!(f, "Generalization of non-ground term"),
            ConfigurationError::InvalidRuleApplication => write!(f, "Invalid rule application"),
            ConfigurationError::EmptyActiveSet => write!(f, "Empty active set"),
            ConfigurationError::InvalidState => write!(f, "Invalid configuration state"),
            ConfigurationError::RuleApplicationError => write!(f, "Rule application error"),
            ConfigurationError::UnknownRule => write!(f, "Trying to apply an unknown rule"),
        }
    }
}

impl std::error::Error for ConfigurationError {}