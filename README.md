# ShindanMaker

[<img alt="github" src="https://img.shields.io/badge/github-araea/shindan_maker-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/araea/shindan-maker)
[<img alt="crates.io" src="https://img.shields.io/crates/v/shindan-maker.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/shindan-maker)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-shindan_maker-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/shindan-maker)

A Rust library for interacting with [ShindanMaker](https://en.shindanmaker.com/), the popular personality quiz service.

## Features

- Asynchronous API
- Multi-domain support (JP, EN, CN, KR, TH)
- Easy shindan submission and result parsing

## Usage

```toml
[dependencies]
shindan-maker = "0.1.6"
```

## Example

```rust
use shindan_maker::{ShindanClient, ShindanDomain, ShindanResult, Segment};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create a client for the English domain
    let client = ShindanClient::new(ShindanDomain::En)?;

    // Shindan ID for "What kind of person will you turn out to be?"
    let shindan_id = "1221154";

    // Get the shindan title
    let title = client.get_title(shindan_id).await?;
    println!("Shindan title: {}", title);

    // Submit the shindan
    let result = client.submit_shindan(shindan_id, "Rust").await?;

    // Process the result
    if let ShindanResult::Text { title, content } = result {
        println!("Result title: {}", title);
        println!("Result content: {:#?}", content);

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
    }

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

