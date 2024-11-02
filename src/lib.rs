/*!
[![GitHub]](https://github.com/araea/shindan-maker)&ensp;[![crates-io]](https://crates.io/crates/shindan-maker)&ensp;[![docs-rs]](crate)

[GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A Rust library for interacting with [ShindanMaker].

This library provides functionality to interact with various ShindanMaker domains, submit shindans, and parse results.

- Asynchronous API (Tokio)
- Multi-domain support (JP, EN, CN, KR, TH)
- Easy shindan submission and result parsing

[ShindanMaker]: https://en.shindanmaker.com/

## Example

### Get title

```rust
use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ShindanClient::new(ShindanDomain::En)?; // Enum variant
    // let client = ShindanClient::new("Jp".parse()?)?; // String slice
    // let client = ShindanClient::new("EN".parse()?)?; // Case-insensitive
    // let client = ShindanClient::new(String::from("cn").parse()?)?; // String

    let title = client
        .get_title("1222992")
        .await?;

    assert_eq!("Fantasy Stats", title);
    Ok(())
}
```

### Get segments (need "segments" feature)

```rust
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() {
    let client = ShindanClient::new(ShindanDomain::En).unwrap();

    let (segments, title) = client
        .get_segments_with_title("1222992", "test_user")
        .await
        .unwrap();

    assert_eq!("Fantasy Stats", title);

    println!("Result title: {}", title);
    println!("Result text: {}", segments);

    println!("Result segments: {:#?}", segments);
}
```

### Get HTML string (need "html" feature)

- HTML string to image: [cdp-html-shot](https://crates.io/crates/cdp-html-shot).

```rust
#[tokio::main]
async fn main() {
    #[cfg(feature = "html")]
    {
        use shindan_maker::{ShindanClient, ShindanDomain};
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let (_html_str, title) = client
            .get_html_str_with_title("1222992", "test_user")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
    }
}
```
*/

mod client;
mod selectors;
mod html_utils;
mod http_utils;
mod shindan_domain;
#[cfg(feature = "segments")]
mod segment;
#[cfg(feature = "html")]
mod html_template;

pub use client::ShindanClient;
pub use shindan_domain::ShindanDomain;
#[cfg(feature = "segments")]
pub use segment::Segment;

#[cfg(test)]
mod tests {
    use crate::{ShindanClient, ShindanDomain};
    use tokio;

    #[tokio::test]
    async fn test_get_title() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let (title, _desc) = client.
            get_title_with_description("1222992")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
    }

    #[cfg(feature = "segments")]
    #[tokio::test]
    async fn test_get_segments() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let (_segments, title) = client
            .get_segments_with_title("1222992", "test_user")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
    }

    #[cfg(feature = "html")]
    #[tokio::test]
    async fn test_get_html_str() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let (_html_str, title) = client
            .get_html_str_with_title("1222992", "test_user")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
    }
}