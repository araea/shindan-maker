//! A Rust library for interacting with Shindan Maker.
//!
//! This library provides functionality to interact with various Shindan Maker domains,
//! submit shindans, and parse results.

mod client;
mod error;
mod segment;

pub use client::{ShindanClient, ShindanDomain, ShindanResult};
pub use error::ShindanError;
pub use segment::{Segment, filter_segments_by_type};

#[cfg(test)]
mod tests {
    use tokio;
    use super::client::{ShindanClient, ShindanDomain};

    #[tokio::test]
    async fn test_get_title() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let result = client.get_title("1218842").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_submit_shindan() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let result = client.submit_shindan("1218842", "test_user").await;
        assert!(result.is_ok());
    }
}