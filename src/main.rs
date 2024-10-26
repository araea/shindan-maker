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

    loop {

    }

    Ok(())
}