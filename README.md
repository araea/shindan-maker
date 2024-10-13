# Shindan Maker

A Rust library for interacting with Shindan Maker, the popular personality quiz service.

## Features

- Multi-domain support (JP, EN, CN, KR, TH)
- Asynchronous API
- Easy shindan submission and result parsing

## Installation

```toml
[dependencies]
shindan-maker = "0.1.0"
```

## Example

Here's a comprehensive example demonstrating how to use the Shindan Maker library:

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
    let result = client.submit_shindan(shindan_id, "Rust Developer").await?;

    // Process the result
    if let ShindanResult::Text { title, content } = result {
        println!("Result title: {}", title);

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

This example shows how to:

1. Create a `ShindanClient`
2. Fetch a shindan title
3. Submit a shindan and get results
4. Process different types of result segments (text and images)
5. Use the `filter_segments_by_type` utility function

## Documentation

For more details, check the [API documentation](https://docs.rs/shindan-maker).

## License

MIT OR Apache-2.0.
