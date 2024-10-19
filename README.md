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

    let shindan_id = "1221154";

    let title = client.get_title(shindan_id).await?;
    println!("Shindan title: {}", title);
}
```

### Get the text result

```rust
use std::error::Error;
use shindan_maker::{ShindanClient, ShindanDomain, ShindanResult, Segment, ShindanTextResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?;

    let shindan_id = "1221154";

    let text_result = client.get_text_result(shindan_id, "test_name").await?;

    let ShindanTextResult { title, content } = text_result;

    println!("Result title: {}", title);
    println!("Result content: {:#?}", content);

    Ok(())
}
```

#### Parse the text result

```rust
use shindan_maker::{ShindanClient, ShindanDomain, ShindanResult, Segment, ShindanTextResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ...

    // Print the result content
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
    
    // Print text segments
    for segment in content.iter().filter_map(|s| s.get_text()) {
        println!("Text: {}", segment);
    }

    // Print image URLs
    for segment in content.iter().filter_map(|s| s.get_image_url()) {
        println!("Image URL: {}", segment);
    }

    // Example of using filter_segments_by_type
    let text_segments: Vec<&Segment> = shindan_maker::filter_segments_by_type(&content, "text");
    println!("Number of text segments: {}", text_segments.len());

    Ok(())
}
```

### Get the image result

```rust
use base64::Engine;
use std::error::Error;
use shindan_maker::{ShindanClient, ShindanDomain, ShindanImageResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = ShindanClient::new(ShindanDomain::En)?.init_browser()?;

    let shindan_id = "1221154";

    let image_result = client.get_image_result(shindan_id, "test_name").await?;

    let ShindanImageResult { title, base64 } = image_result;
    
    let png_data = base64::prelude::BASE64_STANDARD.decode(base64)?;
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

