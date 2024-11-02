use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    const SHINDAN_ID: &str = "1222992";

    let client = ShindanClient::new(ShindanDomain::En)?; // Enum variant
    // let client = ShindanClient::new("Jp".parse()?)?; // String slice
    // let client = ShindanClient::new("EN".parse()?)?; // Case-insensitive
    // let client = ShindanClient::new(String::from("cn").parse()?)?; // String

    let (title, desc) = client
        .get_title_with_description(SHINDAN_ID)
        .await?;

    println!("Title: {}", title);
    println!("Description: {}", desc);

    Ok(())
}