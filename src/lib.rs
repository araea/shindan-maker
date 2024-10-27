/*!
[GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A Rust library for interacting with Shindan Maker.

This library provides functionality to interact with various Shindan Maker domains, submit shindans, and parse results.
*/

mod client;
#[cfg(feature = "segments")]
mod segment;
#[cfg(feature = "html")]
mod html_template;

pub use client::{ShindanClient, ShindanDomain};

#[cfg(feature = "segments")]
pub use segment::Segment;

#[cfg(test)]
mod tests {
    use super::client::{ShindanClient, ShindanDomain};
    use tokio;

    #[tokio::test]
    async fn test_get_title() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let title = client.get_title("1222992").await.unwrap();
        assert_eq!("Reincarnation.", title);
    }

    #[cfg(feature = "segments")]
    #[tokio::test]
    async fn test_get_segments() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (_segments, title) = client.get_segments_with_title("1222992", "test_user").await.unwrap();
        assert_eq!("Reincarnation.", title);
    }

    #[cfg(feature = "html")]
    #[tokio::test]
    async fn test_get_html_str() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (_html_str, title) = client.get_html_str_with_title("1222992", "test_user").await.unwrap();
        assert_eq!("Reincarnation.", title);
    }
}