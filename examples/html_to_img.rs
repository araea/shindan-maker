use std::fs;
use base64::Engine;
use anyhow::Result;
use cdp_html_shot::Browser;
use shindan_maker::{ShindanClient, ShindanDomain};

#[tokio::main]
async fn main() -> Result<()> {
    const SHINDAN_ID: &str = "1223511";
    const USER_NAME: &str = "test_user";

    let client = ShindanClient::new(ShindanDomain::En)?;
    let (html_str, title) = client
        .get_html_str_with_title(SHINDAN_ID, USER_NAME)
        .await?;

    println!("Result title: {}", title);

    let browser = Browser::new().await?;
    let base64 = browser.capture_html(&html_str, "#title_and_result").await?;

    let img_data = base64::prelude::BASE64_STANDARD.decode(base64)?;

    fs::write("test0.jpeg", img_data)?;

    Ok(())
}