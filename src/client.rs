use std::fmt;
use std::time::Duration;
use reqwest::{header, Client};
use serde_json::json;
use tl::Node::{Raw, Tag};
use tl::VDom;
use crate::ShindanError;
use crate::Segment;

/// Represents different Shindan domains.
#[derive(Debug, Clone, Copy)]
pub enum ShindanDomain {
    Jp,
    En,
    Cn,
    Kr,
    Th,
}

impl fmt::Display for ShindanDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

/// Represents the result of a Shindan submission.
#[derive(Debug, Clone)]
pub enum ShindanResult {
    Text {
        title: String,
        content: Vec<Segment>,
    }
}

/// Client for interacting with Shindan Maker.
#[derive(Debug)]
pub struct ShindanClient {
    client: Client,
    domain: ShindanDomain,
}

const TIMEOUT_SECS: u64 = 3;

impl ShindanClient {
    /// Creates a new ShindanClient with the specified domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use shindan_maker::{ShindanClient, ShindanDomain};
    ///
    /// let client = ShindanClient::new(ShindanDomain::En).expect("Failed to create client");
    /// ```
    pub fn new(domain: ShindanDomain) -> Result<Self, reqwest::Error> {
        Ok(Self {
            domain,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()?,
        })
    }

    /// Sets the domain for the ShindanClient.
    pub fn set_domain(&mut self, domain: ShindanDomain) {
        self.domain = domain;
    }

    /// Gets the title of a Shindan by its ID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shindan_maker::{ShindanClient, ShindanDomain};
    /// # use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ShindanClient::new(ShindanDomain::En)?;
    ///     let title = client.get_title("1218842").await?;
    ///     println!("Shindan title: {}", title);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_title(&self, id: &str) -> Result<String, ShindanError> {
        let url = format!("{}{}", self.domain, id);
        let response = self.client.get(&url).send().await?;
        let text = response.text().await?;
        let document = tl::parse(&text, tl::ParserOptions::default())
            .map_err(|e| ShindanError::Parse(e.to_string()))?;
        self.extract_title(&document)
    }

    /// Submits a Shindan and returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use shindan_maker::{ShindanClient, ShindanDomain, ShindanResult};
    /// # use tokio;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = ShindanClient::new(ShindanDomain::En)?;
    ///     let result = client.submit_shindan("1218842", "test_user").await?;
    ///     if let ShindanResult::Text { title, content } = result {
    ///         println!("Title: {}", title);
    ///         for segment in content {
    ///             if let Some(text) = segment.get_text() {
    ///                 println!("Text: {}", text);
    ///             }
    ///         }
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn submit_shindan(
        &self,
        id: &str,
        name: &str,
    ) -> Result<ShindanResult, ShindanError> {
        let url = format!("{}{}", self.domain, id);
        let initial_response = self.client.get(&url).send().await?;
        let session_cookie = Self::extract_session_cookie(&initial_response)?;
        let initial_response_text = initial_response.text().await?;
        let initial_document = tl::parse(&initial_response_text, tl::ParserOptions::default())
            .map_err(|e| ShindanError::Parse(e.to_string()))?;

        let title = self.extract_title(&initial_document)?;
        let form_data = ShindanClient::extract_form_data(&initial_document, name)?;

        let headers = self.prepare_headers(&session_cookie)?;
        let response_text = self.client
            .post(&url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await?;

        let result_document = tl::parse(&response_text, tl::ParserOptions::default())
            .map_err(|e| ShindanError::Parse(e.to_string()))?;
        self.parse_result(&result_document, title)
    }

    fn extract_title(&self, dom: &VDom) -> Result<String, ShindanError> {
        let parser = dom.parser();
        let element = dom.get_element_by_id("shindanTitle")
            .ok_or_else(|| ShindanError::Parse("Title not found".into()))?
            .get(parser)
            .unwrap();

        Ok(element.inner_text(parser).to_string())
    }

    fn extract_session_cookie(response: &reqwest::Response) -> Result<String, ShindanError> {
        response.cookies()
            .find(|cookie| cookie.name() == "_session")
            .map(|cookie| cookie.value().to_string())
            .ok_or(ShindanError::SessionCookieNotFound)
    }

    fn extract_form_data(
        dom: &VDom,
        name: &str,
    ) -> Result<Vec<(&'static str, String)>, ShindanError> {
        const FIELDS: [&str; 4] = ["_token", "randname", "type", "shindan_token"];
        let parser = dom.parser();
        let mut form_data = Vec::with_capacity(FIELDS.len() + 1);

        let get_input_value = |field: &'static str| -> Result<String, ShindanError> {
            let selector = format!("input[name=\"{}\"]", field);
            let input = dom
                .query_selector(&selector)
                .ok_or_else(|| ShindanError::Parse(format!("Invalid selector: {}", selector)))?
                .next()
                .ok_or_else(|| ShindanError::TokenNotFound(field))?;

            let tag = input
                .get(parser)
                .ok_or_else(|| ShindanError::Parse("Failed to get element".into()))?
                .as_tag()
                .ok_or_else(|| ShindanError::Parse("Element is not a tag".into()))?;

            let value = tag
                .attributes()
                .get("value")
                .flatten()
                .ok_or_else(|| ShindanError::TokenNotFound(field))?;

            Ok(value.as_utf8_str().to_string())
        };

        for &field in &FIELDS {
            match get_input_value(field) {
                Ok(value) => form_data.push((field, value)),
                Err(e) => return Err(e),
            }
        }

        form_data.push(("user_input_value_1", name.to_string()));

        Ok(form_data)
    }

    fn prepare_headers(&self, session_cookie: &str) -> Result<header::HeaderMap, ShindanError> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let cookie_value = format!("_session={};", session_cookie);
        headers.insert(
            header::COOKIE,
            header::HeaderValue::from_str(&cookie_value)
                .map_err(|_| ShindanError::Parse("Failed to create cookie header".into()))?,
        );

        Ok(headers)
    }

    fn parse_result(&self, dom: &VDom, title: String) -> Result<ShindanResult, ShindanError> {
        let parser = dom.parser();

        let element = dom.get_element_by_id("post_display")
            .expect("Failed to find element")
            .get(parser)
            .unwrap();

        if let Tag(tag) = element {
            let mut content = Vec::new();
            for child in tag.children().all(parser) {
                match child {
                    Raw(bytes) => content.push(Segment::new("text", json!({
                            "text": bytes.as_utf8_str().to_string().replace("&nbsp;", " ")
                        }))),
                    Tag(html_tag) if html_tag.name() == "br" => content.push(Segment::new("text", json!({
                            "text": "\n".to_string()
                        }))),
                    Tag(html_tag) if html_tag.name() == "img" => content.push(Segment::new("image", json!({
                            "file": html_tag.attributes().get("data-src").flatten().unwrap().as_utf8_str().to_string()
                        }))),
                    _ => {}
                }
            }

            return Ok(ShindanResult::Text { title, content });
        }

        Err(ShindanError::Parse("Failed to parse result".into()))
    }
}