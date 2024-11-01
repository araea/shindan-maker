use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    const SHINDAN_ID: &str = "1222992";

    let client = ShindanClient::new(ShindanDomain::En)?; // Enum variant
    // let client = ShindanClient::new("Jp".parse()?)?; // String slice
    // let client = ShindanClient::new("EN".parse()?)?; // Case-insensitive
    // let client = ShindanClient::new(String::from("cn").parse()?)?; // String

    let title = client
        .get_title(SHINDAN_ID)
        .await?;

    println!("Title: {}", title);

    Ok(())
}