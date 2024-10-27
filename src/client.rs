use std::fmt;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use reqwest::{header, Client};
use serde_json::json;
use anyhow::{Context, Result};
use scraper::{Html, Node, Selector};

#[cfg(feature = "segments")]
use crate::segment::{Segment, Segments};

#[cfg(feature = "html")]
use anyhow::anyhow;
#[cfg(feature = "html")]
use crate::html_template::HTML_TEMPLATE;

/// A domain of Shindan Maker.
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

#[derive(Clone)]
struct Selectors {
    form: Vec<Selector>,
    shindan_title: Selector,

    #[cfg(feature = "segments")]
    post_display: Selector,

    #[cfg(feature = "html")]
    title_and_result: Selector,
    #[cfg(feature = "html")]
    script: Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
            shindan_title: Selector::parse("#shindanTitle").expect("Failed to parse selector"),

            #[cfg(feature = "segments")]
            post_display: Selector::parse("#post_display").expect("Invalid selector"),

            #[cfg(feature = "html")]
            title_and_result: Selector::parse("#title_and_result").expect("Failed to parse selector"),
            #[cfg(feature = "html")]
            script: Selector::parse("script").expect("Invalid script selector"),

            form: vec![
                Selector::parse("input[name=_token]").expect("Failed to parse selector"),
                Selector::parse("input[name=randname]").expect("Failed to parse selector"),
                Selector::parse("input[name=type]").expect("Failed to parse selector"),
            ],
        }
    }
}

/// A client for interacting with Shindan Maker.
#[derive(Clone)]
pub struct ShindanClient {
    client: Client,
    domain: ShindanDomain,
    selectors: Selectors,
}

const TIMEOUT_SECS: u64 = 3;

impl ShindanClient {
    /**
    Create a new Shindan Maker client.

    # Arguments
    - `domain` - The domain of Shindan Maker to use.

    # Returns
    A new Shindan Maker client.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    let client = ShindanClient::new(ShindanDomain::En).unwrap();
    ```
    */
    pub fn new(domain: ShindanDomain) -> Result<Self> {
        Ok(Self {
            domain,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()?,
            selectors: Selectors::new(),
        })
    }

    /**
    Get the title of a shindan.

    # Arguments
    - `id` - The ID of the shindan.

    # Returns
    The title of the shindan.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let title = client.get_title("1222992").await.unwrap();
        assert_eq!("Reincarnation.", title);
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
        self.extract_title(&document)
    }

    /**
    Get the segments of a shindan.

    # Arguments
    - `id` - The ID of the shindan.
    - `name` - The name to use for the shindan.

    # Returns
    The segments of the shindan and the title of the shindan.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (segments, title) = client.get_segments_with_title("1222992", "test_user").await.unwrap();
        assert_eq!("Reincarnation.", title);

        println!("Result title: {}", title);
        println!("Result segments: {:#?}", segments);
        println!("Result text: {}", segments);
    }
    ```
    */
    #[cfg(feature = "segments")]
    pub async fn get_segments_with_title(
        &self,
        id: &str,
        name: &str,
    ) -> Result<(Segments, String)> {
        let (title, response_text) = self.get_title_and_init_res(id, name).await?;

        let result_document = Html::parse_document(&response_text);

        let mut segments = Vec::new();

        result_document.select(&self.selectors.post_display)
            .next()
            .context("Failed to get the next element")?
            .children()
            .for_each(|child| {
                let node = child.value();
                match node {
                    Node::Text(text) => {
                        let text = text.replace("&nbsp;", " ");
                        segments.push(Segment::new("text", json!({
                            "text": text
                        })));
                    }
                    Node::Element(element) => {
                        if element.name() == "br" {
                            let text = "\n".to_string();
                            segments.push(Segment::new("text", json!({
                                "text": text
                            })));
                        } else if element.name() == "img" {
                            let image_url = element.attr("data-src").expect("Failed to get 'data-src' attribute").to_string();
                            segments.push(Segment::new("image", json!({
                                "file": image_url
                            })));
                        }
                    }
                    _ => {}
                }
            });

        Ok((Segments(segments), title))
    }

    /**
    Get the HTML string of a shindan.

    # Arguments
    - `id` - The ID of the shindan.
    - `name` - The name to use for the shindan.

    # Returns
    The HTML string of the shindan and the title of the shindan.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (_html_str, title) = client.get_html_str_with_title("1222992", "test_user").await.unwrap();
        assert_eq!("Reincarnation.", title);
    }
    ```
    */
    #[cfg(feature = "html")]
    pub async fn get_html_str_with_title(
        &self,
        id: &str,
        name: &str,
    ) -> Result<(String, String)> {
        let (title, response_text) = self.get_title_and_init_res(id, name).await?;

        let html = self.get_html_string(id, &response_text)?;

        Ok((html, title))
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

        let (title, form_data) = self.extract_title_and_form_data(&initial_response_text, name)?;

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

    fn extract_title_and_form_data(&self, html_content: &str, name: &str) -> Result<(String, Vec<(&'static str, String)>)> {
        let document = Html::parse_document(html_content);
        let title = self.extract_title(&document)?;
        let form_data = self.extract_form_data(&document, name)?;

        Ok((title, form_data))
    }

    fn extract_title(&self, dom: &Html) -> Result<String> {
        Ok(dom
            .select(&self.selectors.shindan_title)
            .next()
            .context("Failed to get the next element")?
            .value().attr("data-shindan_title")
            .context("Failed to get 'data-shindan_title' attribute")?
            .to_string())
    }

    fn extract_session_cookie(response: &reqwest::Response) -> Result<String> {
        response.cookies()
            .find(|cookie| cookie.name() == "_session")
            .map(|cookie| cookie.value().to_string())
            .context("Failed to extract session cookie")
    }

    fn extract_form_data(
        &self,
        dom: &Html,
        name: &str,
    ) -> Result<Vec<(&'static str, String)>> {
        const FIELDS: &[&str] = &["_token", "randname", "type"];
        let mut form_data = Vec::with_capacity(FIELDS.len() + 1);

        for (index, &field) in FIELDS.iter().enumerate() {
            let value = dom.select(&self.selectors.form[index])
                .next()
                .context("Failed to get the next element")?
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

    #[cfg(feature = "html")]
    fn get_html_string(&self, id: &str, response_text: &str) -> Result<String> {
        let result_document = Html::parse_document(response_text);

        let title_and_result = result_document
            .select(&self.selectors.title_and_result)
            .next()
            .context("Failed to get the next element")?
            .html();

        let mut html = HTML_TEMPLATE
            .replace("<!-- TITLE_AND_RESULT -->", &title_and_result);

        if response_text.contains("chart.js") {
            let mut scripts = vec![
                r#"<script src="https://cn.shindanmaker.com/js/app.js?id=163959a7e23bfa7264a0ddefb3c36f13" defer=""></script>"#,
                r#"<script src="https://cn.shindanmaker.com/js/chart.js?id=391e335afc72362acd6bf1ea1ba6b74c" defer=""></script>"#];

            let shindan_script = self.get_first_script(&result_document, id)?;
            scripts.push(&shindan_script);
            html = html.replace("<!-- SCRIPTS -->", &scripts.join("\n"));
        }
        Ok(html)
    }

    #[cfg(feature = "html")]
    fn get_first_script(&self, result_document: &Html, id: &str) -> Result<String> {
        for element in result_document.select(&self.selectors.script) {
            let html = element.html();
            if html.contains(id) {
                return Ok(html);
            }
        }

        Err(anyhow!("Failed to find script with id {}", id))
    }
}
