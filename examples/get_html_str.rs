use anyhow::Result;
use shindan_maker::{ShindanClient, ShindanDomain};
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let shindan_ids = [
        ("1252750", "人设生成"),
        ("1222992", "Fantasy Stats"),
        ("1253379", "一封给新生魔女的建议信"),
        ("1253257", "你会拥有怎样的异世界旅程？"),
        ("1150916", "哥们考试"),
        ("1252510", "图表组件"),
    ];

    const USER_NAME: &str = "test_user";
    let client = ShindanClient::new(ShindanDomain::Jp)?;

    for (shindan_id, desc) in shindan_ids.iter() {
        let output_file = format!("shindan_{}.html", shindan_id);

        if Path::new(&output_file).exists() {
            println!(
                "Warning: {} already exists, will be overwritten",
                output_file
            );
        }

        let (html_str, title) = client
            .get_html_str_with_title(shindan_id, USER_NAME)
            .await?;

        println!("Result for [{} - {}]: {}", shindan_id, desc, title);

        save_to_file(&html_str, &output_file)?;
    }

    Ok(())
}

fn save_to_file(content: &str, filename: &str) -> Result<()> {
    fs::write(filename, content)?;
    println!("Content saved to {}", filename);
    Ok(())
}
