//! Error types for the Solana Stream SDK

use thiserror::Error;

/// Main error type for the Solana Stream SDK
#[derive(Error, Debug)]
pub enum SolanaStreamError {
    /// Tonic transport error
    #[error("Transport error: {0}")]
    Transport(#[from] tonic::transport::Error),

    /// Tonic status error
    #[error("gRPC status error: {0}")]
    Status(#[from] tonic::Status),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
}
