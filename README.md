# ShindanMaker

[<img alt="github" src="https://img.shields.io/badge/github-araea/shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/araea/shindan-maker)
[<img alt="crates.io" src="https://img.shields.io/crates/v/shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/shindan-maker)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-shindan_maker-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/shindan-maker)

A Rust library for interacting with [ShindanMaker](https://en.shindanmaker.com/), the popular personality quiz service.

## Features

- Asynchronous API
- Multi-domain support (JP, EN, CN, KR, TH)
- Easy shindan submission and result parsing
- Image result support (optional, headless browser required)

## Usage

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }

# For full functionality
shindan-maker = { version = "0.1", features = ["full"] }

# For text-only functionality
# shindan-maker = "0.1"

# For image functionality
# shindan-maker = { version = "0.1", features = ["image"] }
```

## Example

### Get the shindan title

```rust
use std::error::Error;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?;
    let title = client.get_title("1218842").await?;
    println!("Title: {}", title);
    Ok(())
}
```

### Get the text result

```rust
use std::error::Error;
use shindan_maker::{ShindanClient, ShindanDomain, ShindanTextResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?;
    let result = client.get_text_result("1218842", "test_user").await?;

    let ShindanTextResult { title, content } = result;
    println!("Result title: {}", title);
    println!("Result content: {:#?}", content);

    Ok(())
}
```

#### Printing text segments

```rust
use shindan_maker::{ShindanClient, ShindanDomain, ShindanTextResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?;
    let result = client.get_text_result("1218842", "test_user").await?;

    let ShindanTextResult { title, content } = result;

    println!("Result title: {}", title);

    let mut text = String::new();
    for segment in content.iter() {
        match segment.type_.as_str() {
            "text" => {
                text.push_str(&segment.get_text().unwrap());
            }
            "image" => {
                text.push_str(&segment.get_image_url().unwrap());
            }
            _ => {}
        }
    }
    println!("Result text: {}", text);

    Ok(())
}
```

#### Filtering segments by type

```rust
use shindan_maker::{ShindanClient, ShindanDomain, filter_segments_by_type};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?;
    let result = client.get_text_result("1218842", "test_user").await?;

    println!("Result title: {}", result.title);

    let text_segments = filter_segments_by_type(&result.content, "text");
    assert_eq!(text_segments.len(), 2);

    Ok(())
}
```

### Get the image result

```rust
use std::error::Error;
use base64::Engine;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?.init_browser()?;
    let result = client.get_image_result("1218842", "test_user").await?;

    println!("Result title: {}", result.title);

    let png_data = base64::prelude::BASE64_STANDARD.decode(result.base64)?;
    std::fs::write("test.png", png_data)?;

    Ok(())
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

