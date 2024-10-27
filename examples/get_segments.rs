use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    const SHINDAN_ID: &str = "1222992";
    const USER_NAME: &str = "test_user";

    let client = ShindanClient::new(ShindanDomain::En)?;
    let (segments, title) = client
        .get_segments_with_title(SHINDAN_ID, USER_NAME)
        .await?;

    println!("Result segments: {:#?}", segments);

    println!("Result title: {}", title);
    println!("Result text: {}", segments);

    Ok(())
}