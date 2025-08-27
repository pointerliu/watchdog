//! Manager traits and implementations for fetchers and notifiers using actor pattern

/// Trait defining the interface for a manager
pub trait Manager: Send + Sync {
    /// Start the manager
    fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Stop the manager
    fn stop(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
