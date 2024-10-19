use std::fmt;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use reqwest::{header, Client};
use serde_json::json;
use anyhow::{Context, Result};
use scraper::{Html, Node, Selector};
#[cfg(feature = "image")]
use anyhow::anyhow;

use crate::Segment;

#[cfg(feature = "image")]
use crate::html_template::HTML_TEMPLATE;
#[cfg(feature = "image")]
use headless_chrome::{Browser, LaunchOptions};
#[cfg(feature = "image")]
use headless_chrome::browser::default_executable;
#[cfg(feature = "image")]
use headless_chrome::protocol::cdp::Page;

/// Represents a Shindan Maker domain.
#[derive(Debug, Clone, Copy)]
pub enum ShindanDomain {
    Jp,
    En,
    Cn,
    Kr,
    Th,
}

impl fmt::Display for ShindanDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let url = match self {
            Self::Jp => "https://shindanmaker.com/",
            Self::En => "https://en.shindanmaker.com/",
            Self::Cn => "https://cn.shindanmaker.com/",
            Self::Kr => "https://kr.shindanmaker.com/",
            Self::Th => "https://th.shindanmaker.com/",
        };
        write!(f, "{}", url)
    }
}

/// Represents the text result of a shindan.
#[derive(Debug, Clone)]
pub struct ShindanTextResult {
    pub title: String,
    pub content: Vec<Segment>,
}

/// Represents the image result of a shindan.
#[cfg(feature = "image")]
#[derive(Debug, Clone)]
pub struct ShindanImageResult {
    pub title: String,
    pub base64: String,
}

/// A client for interacting with Shindan Maker.
#[derive(Clone)]
pub struct ShindanClient {
    client: Client,
    #[cfg(feature = "image")]
    browser: Option<Browser>,
    domain: ShindanDomain,
}

const TIMEOUT_SECS: u64 = 3;

impl ShindanClient {
    /**
    Creates a new ShindanClient with the specified domain.

    # Arguments

    * `domain` - The domain to use.

    # Examples

    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En);
        assert!(client.is_ok());
    }
    ```
    */
    pub fn new(domain: ShindanDomain) -> Result<Self> {
        Ok(Self {
            domain,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()?,
            #[cfg(feature = "image")]
            browser: None,
        })
    }

    /**
    Initializes the browser for image results.

    # Examples

    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?.init_browser();
        assert!(client.is_ok());

        Ok(())
    }
    ```
    */
    #[cfg(feature = "image")]
    pub fn init_browser(mut self) -> Result<Self> {
        let launch_options = LaunchOptions::default_builder()
            .args(vec![
                "--window-position=-2400,-2400".as_ref(),
            ])
            .path(Some(default_executable()
                .map_err(|e| anyhow!(e))?))
            .build()?;

        self.browser = Some(Browser::new(launch_options)?);
        Ok(self)
    }

    /**
    Gets the title of a shindan with the specified ID.

    # Arguments

    * `id` - The ID of the shindan.

    # Examples

    ```
    use std::error::Error;
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let title = client.get_title("1218842").await?;
        println!("Title: {}", title);
        Ok(())
    }
    ```
    */
    pub async fn get_title(&self, id: &str) -> Result<String> {
        let url = format!("{}{}", self.domain, id);

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let text = response
            .text()
            .await?;

        let document = Html::parse_document(&text);
        Self::extract_title(&document)
    }

    /**
    Gets the text result of a shindan with the specified ID and name.

    # Arguments

    * `id` - The ID of the shindan.
    * `name` - The name to use.

    # Examples

    ## Basic usage

    ```
    use std::error::Error;
    use shindan_maker::{ShindanClient, ShindanDomain, ShindanTextResult};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let result = client.get_text_result("1218842", "test_user").await?;

        let ShindanTextResult { title, content } = result;
        println!("Result title: {}", title);
        println!("Result content: {:#?}", content);

        Ok(())
    }
    ```

    ## Printing text segments

    ```
    use shindan_maker::{ShindanClient, ShindanDomain, ShindanTextResult};

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let result = client.get_text_result("1218842", "test_user").await?;

        let ShindanTextResult { title, content } = result;

        println!("Result title: {}", title);

        let mut text = String::new();
        for segment in content.iter() {
            match segment.type_.as_str() {
                "text" => {
                    text.push_str(&segment.get_text().unwrap());
                }
                "image" => {
                    text.push_str(&segment.get_image_url().unwrap());
                }
                _ => {}
            }
        }
        println!("Result text: {}", text);

        Ok(())
    }
    ```

    ## Filtering segments by type

    ```
    use shindan_maker::{ShindanClient, ShindanDomain, filter_segments_by_type};
    use serde_json::json;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let client = ShindanClient::new(ShindanDomain::En)?;
        let result = client.get_text_result("1218842", "test_user").await?;

        println!("Result title: {}", result.title);

        let text_segments = filter_segments_by_type(&result.content, "text");
        assert_eq!(text_segments.len(), 2);

        Ok(())
    }
    ```
    */
    pub async fn get_text_result(
        &self,
        id: &str,
        name: &str,
    ) -> Result<ShindanTextResult> {
        let (title, response_text) = self.get_title_and_init_res(id, name).await?;

        let result_document = Html::parse_document(&response_text);

        let mut content = Vec::new();

        result_document.select(&Selector::parse("#post_display").expect("Invalid selector"))
            .next()
            .context("Failed to get next element")?
            .children()
            .for_each(|child| {
                let node = child.value();
                match node {
                    Node::Text(text) => {
                        let text = text.replace("&nbsp;", " ");
                        content.push(Segment::new("text", json!({
                            "text": text
                        })));
                    }
                    Node::Element(element) => {
                        if element.name() == "br" {
                            let text = "\n".to_string();
                            content.push(Segment::new("text", json!({
                                "text": text
                            })));
                        } else if element.name() == "img" {
                            let image_url = element.attr("data-src").expect("Failed to get 'data-src' attribute").to_string();
                            content.push(Segment::new("image", json!({
                                "file": image_url
                            })));
                        }
                    }
                    _ => {}
                }
            });

        Ok(ShindanTextResult { title, content })
    }

    /**
    Gets the image result of a shindan with the specified ID and name.

    # Arguments

    * `id` - The ID of the shindan.
    * `name` - The name to use.

    # Examples

    ```
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

        Ok(())
    }
    ```
    */
    #[cfg(feature = "image")]
    pub async fn get_image_result(
        &self,
        id: &str,
        name: &str,
    ) -> Result<ShindanImageResult> {
        if self.browser.is_none() {
            Err(anyhow!("Browser not initialized"))?;
        }

        let (title, response_text) = self.get_title_and_init_res(id, name).await?;

        let html = Self::get_html_string(id, &response_text)?;

        let tab = self.browser
            .as_ref()
            .unwrap()
            .new_tab()?;

        let expression = format!("document.open();
            document.write(String.raw`{}`);
            document.close();", html);
        tab.evaluate(&expression, false).context("Failed to evaluate expression")?;
        tab.wait_until_navigated()?;

        let element = tab.find_element("#title_and_result")?;

        let base64 = element.parent.call_method(Page::CaptureScreenshot {
            format: Some(Page::CaptureScreenshotFormatOption::Png),
            clip: Some(element.get_box_model()?.border_viewport()),
            quality: Some(90),
            from_surface: Some(true),
            capture_beyond_viewport: Some(true),
        })?
            .data;

        tab.close(false)?;

        Ok(ShindanImageResult {
            title,
            base64,
        })
    }

    async fn get_title_and_init_res(&self, id: &str, name: &str) -> Result<(String, String)> {
        let url = format!("{}{}", self.domain, id);
        let initial_response = self.client
            .get(&url)
            .send()
            .await?;

        let session_cookie = Self::extract_session_cookie(&initial_response)?;
        let initial_response_text = initial_response
            .text()
            .await?;

        let (title, form_data) = Self::extract_title_and_form_data(&initial_response_text, name)?;

        let headers = Self::prepare_headers(&session_cookie)?;
        let response_text = self.client
            .post(&url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await?;

        Ok((title, response_text))
    }

    fn extract_title_and_form_data(html_content: &str, name: &str) -> Result<(String, Vec<(&'static str, String)>)> {
        let document = Html::parse_document(html_content);
        let title = Self::extract_title(&document)?;
        let form_data = Self::extract_form_data(&document, name)?;

        Ok((title, form_data))
    }

    fn extract_title(dom: &Html) -> Result<String> {
        Ok(dom
            .select(&Selector::parse("#shindanTitle").expect("Failed to parse selector"))
            .next()
            .context("Failed to get next element")?
            .value().attr("data-shindan_title").context("Failed to get 'data-shindan_title' attribute")?.to_string())
    }

    fn extract_session_cookie(response: &reqwest::Response) -> Result<String> {
        response.cookies()
            .find(|cookie| cookie.name() == "_session")
            .map(|cookie| cookie.value().to_string())
            .context("Failed to extract session cookie")
    }

    fn extract_form_data(
        dom: &Html,
        name: &str,
    ) -> Result<Vec<(&'static str, String)>> {
        const FIELDS: &[&str] = &["_token", "randname", "type"];
        let mut form_data = Vec::with_capacity(FIELDS.len() + 1);

        for &field in FIELDS {
            let selector = format!("input[name={field}]", field = field);
            let value = dom.select(&Selector::parse(&selector).expect("Failed to parse selector"))
                .next()
                .context("Failed to get next element")?
                .value()
                .attr("value")
                .context("Failed to get value attribute")?;

            form_data.push((field, value.to_string()));
        }

        form_data.push(("user_input_value_1", name.to_string()));

        Ok(form_data)
    }

    fn prepare_headers(session_cookie: &str) -> Result<header::HeaderMap> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let cookie_value = format!("_session={};", session_cookie);
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie_value)?,
        );

        Ok(headers)
    }

    #[cfg(feature = "image")]
    fn get_html_string(id: &str, response_text: &str) -> Result<String> {
        let result_document = Html::parse_document(response_text);

        let title_and_result = result_document
            .select(&Selector::parse("#title_and_result").expect("Failed to parse selector"))
            .next()
            .context("Failed to get next element")?
            .html();

        let mut html = HTML_TEMPLATE
            .replace("<!-- TITLE_AND_RESULT -->", &title_and_result);

        if response_text.contains("chart.js") {
            let mut scripts = vec![
                r#"<script src="https://cn.shindanmaker.com/js/app.js?id=163959a7e23bfa7264a0ddefb3c36f13" defer=""></script>"#,
                r#"<script src="https://cn.shindanmaker.com/js/chart.js?id=391e335afc72362acd6bf1ea1ba6b74c" defer=""></script>"#];

            let shindan_script = Self::get_first_script(&result_document, id)?;
            scripts.push(&shindan_script);
            html = html.replace("<!-- SCRIPTS -->", &scripts.join("\n"));
        }
        Ok(html)
    }

    #[cfg(feature = "image")]
    fn get_first_script(result_document: &Html, id: &str) -> Result<String> {
        let selector = Selector::parse("script").expect("Invalid script selector");

        for element in result_document.select(&selector) {
            let html = element.html();
            if html.contains(id) {
                return Ok(html);
            }
        }

        Err(anyhow!("Failed to find script with id {}", id))
    }
}
