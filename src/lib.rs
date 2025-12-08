/*!
[![GitHub]](https://github.com/araea/shindan-maker)&ensp;[![crates-io]](https://crates.io/crates/shindan-maker)&ensp;[![docs-rs]](crate)

[GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A Rust library for interacting with [ShindanMaker].
*/

pub mod client;
pub mod domain;
mod internal;

#[cfg(feature = "segments")]
pub mod models;

// Re-exports for convenient access
pub use client::ShindanClient;
pub use domain::ShindanDomain;

#[cfg(feature = "segments")]
pub use models::{Segment, Segments};

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_title() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (title, _) = client.get_title_with_description("1222992").await.unwrap();
        assert_eq!("Fantasy Stats", title);
    }
}
