# ShindanMaker

[<img alt="github" src="https://img.shields.io/badge/github-araea/shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/araea/shindan-maker)
[<img alt="crates.io" src="https://img.shields.io/crates/v/shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/shindan-maker)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-shindan_maker-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/shindan-maker)

A Rust library for interacting with [ShindanMaker](https://en.shindanmaker.com/), the popular personality quiz service.

- Asynchronous API (Tokio)
- Multi-domain support (JP, EN, CN, KR, TH)
- Easy shindan submission and result parsing

## Usage

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# default feature: ["segments"]
# optional features: ["html"], ["full"](segments + html)
shindan-maker = { version = "0.1", features = ["segments"] }
```

## Example

### Get title

```rust
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() {
    let client = ShindanClient::new(ShindanDomain::En).unwrap();
    
    let title = client
        .get_title("1222992")
        .await
        .unwrap();
    
    assert_eq!("Reincarnation.", title);
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
    
    assert_eq!("Reincarnation.", title);

    println!("Result title: {}", title);
    println!("Result text: {}", segments);
    
    println!("Result segments: {:#?}", segments);
}
```

### Get HTML string (need "html" feature)

```rust
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() {
    let client = ShindanClient::new(ShindanDomain::En).unwrap();
    
    let (_html_str, title) = client
        .get_html_str_with_title("1222992", "test_user")
        .await
        .unwrap();
    
    assert_eq!("Reincarnation.", title);
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

