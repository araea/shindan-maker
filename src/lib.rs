/*!
[GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A Rust library for interacting with Shindan Maker.

This library provides functionality to interact with various Shindan Maker domains, submit shindans, and parse results.
*/

mod client;
mod segment;
#[cfg(feature = "image")]
mod html_template;

pub use client::{ShindanClient, ShindanDomain, ShindanTextResult};
pub use segment::{Segment, filter_segments_by_type};
#[cfg(feature = "image")]
pub use client::ShindanImageResult;

#[cfg(test)]
mod tests {
    use tokio;
    use super::client::{ShindanClient, ShindanDomain};

    #[tokio::test]
    async fn test_get_title() -> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let result = client.get_title("1221154").await?;
        assert_eq!("What kind of a person will you turn out to be?", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_text_result() -> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let result = client.get_text_result("1221154", "test_user").await?;
        assert_eq!("What kind of a person will you turn out to be?", result.title);
        Ok(())
    }

    #[cfg(feature = "image")]
    #[tokio::test]
    async fn test_get_image_result()-> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?.init_browser()?;
        let result = client.get_image_result("1221154", "test_user").await?;
        assert_eq!("What kind of a person will you turn out to be?", result.title);
        Ok(())
    }
}