#[derive(Debug, Clone)]
pub enum CompositionError {
    CompositionFailure,
    CompositionTimeout,
    UniqueGatePropertyUnsatisfied,
    MergeFailure,
}