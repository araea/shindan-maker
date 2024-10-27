use std::error::Error;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const SHINDAN_ID: &str = "1222992";

    let client = ShindanClient::new(ShindanDomain::En)?;
    let title = client
        .get_title(SHINDAN_ID)
        .await?;

    println!("Title: {}", title);
    Ok(())
}