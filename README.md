# ShindanMaker

[![github](https://img.shields.io/badge/github-araea/shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/araea/shindan-maker)
[![crates.io](https://img.shields.io/crates/v/shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/shindan-maker)
[![docs.rs](https://img.shields.io/badge/docs.rs-shindan_maker-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/shindan-maker)

A Rust library for interacting with [ShindanMaker](https://en.shindanmaker.com/), the popular personality quiz service.

- Asynchronous API (Tokio)
- Multi-domain support (JP, EN, CN, KR, TH)
- Easy shindan submission and result parsing

## Usage

```toml
[dependencies]
# default feature:   ["segments"]
# optional features: ["html"], ["full"] (segments + html)
shindan-maker = { version = "0.1", features = ["segments"] }
```

## Example

### Get title

```rust
use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ShindanClient::new(ShindanDomain::En)?;               // Enum variant
    // let client = ShindanClient::new("Jp".parse()?)?;                 // String slice
    // let client = ShindanClient::new("EN".parse()?)?;                 // Case-insensitive
    // let client = ShindanClient::new(String::from("cn").parse()?)?;   // String

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

To convert the HTML string to an image, consider using [cdp-html-shot](https://crates.io/crates/cdp-html-shot).

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

---

#### License

_Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option._

_Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions._
