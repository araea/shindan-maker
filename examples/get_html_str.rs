use std::error::Error;
use std::fs;
use std::path::Path;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const SHINDAN_ID: &str = "1222992";
    const USER_NAME: &str = "test_user";
    const OUTPUT_FILE: &str = "shindan.html";

    if Path::new(OUTPUT_FILE).exists() {
        println!("Warning: {} already exists, will be overwritten", OUTPUT_FILE);
    }

    let client = ShindanClient::new(ShindanDomain::En)?;
    let (html_str, title) = client
        .get_html_str_with_title(SHINDAN_ID, USER_NAME)
        .await?;

    println!("Result title: {}", title);

    save_to_file(&html_str, OUTPUT_FILE)?;

    Ok(())
}

fn save_to_file(content: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    fs::write(filename, content)?;
    println!("Content saved to {}", filename);
    Ok(())
}