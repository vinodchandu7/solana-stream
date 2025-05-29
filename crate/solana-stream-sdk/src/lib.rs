//! # Solana Stream SDK
//!
//! A Rust SDK for streaming Solana data using Jito protocols.
//! This crate provides convenient wrappers around the Jito protobuf definitions
//! for easier integration with Solana streaming services.

pub mod error;
pub mod shredstream;

// Internal protobuf modules
pub mod shared {
    tonic::include_proto!("shared");
}

pub mod shredstream_proto {
    tonic::include_proto!("shredstream");
}

// Re-export commonly used types for convenience
// Re-export error types
pub use error::SolanaStreamError;
// Re-export shredstream client
pub use shredstream::ShredstreamClient;
pub use shredstream_proto::{
    CommitmentLevel, SubscribeEntriesRequest, SubscribeRequestFilterAccounts,
    SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterLamports,
    SubscribeRequestFilterAccountsFilterMemcmp, SubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions,
};

pub type Result<T> = std::result::Result<T, SolanaStreamError>;
