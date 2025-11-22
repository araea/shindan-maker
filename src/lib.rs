/*!
[![GitHub]](https://github.com/araea/shindan-maker)&ensp;[![crates-io]](https://crates.io/crates/shindan-maker)&ensp;[![docs-rs]](crate)

[GitHub]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs

<br>

A Rust library for interacting with [ShindanMaker].
*/

use anyhow::{Context, Result};
use reqwest::{Client, header};
use scraper::Html;
use std::str::FromStr;
use std::time::Duration;

// Re-exports
pub use domain::ShindanDomain;

#[cfg(feature = "segments")]
pub use models::{Segment, Segments};

/// A client for interacting with ShindanMaker.
#[derive(Clone, Debug)]
pub struct ShindanClient {
    client: Client,
    domain: ShindanDomain,
}

impl ShindanClient {
    /// Create a new ShindanMaker client.
    pub fn new(domain: ShindanDomain) -> Result<Self> {
        const TIMEOUT_SECS: u64 = 3;
        const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

        Ok(Self {
            domain,
            client: Client::builder()
                .user_agent(USER_AGENT)
                .use_rustls_tls()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .cookie_store(true) // Reqwest handles cookies natively now
                .build()?,
        })
    }

    /// Fetches and extracts title from a shindan page.
    pub async fn get_title(&self, id: &str) -> Result<String> {
        let document = self.fetch_document(id).await?;
        internal::extract_title(&document)
    }

    /// Fetches and extracts description from a shindan page.
    pub async fn get_description(&self, id: &str) -> Result<String> {
        let document = self.fetch_document(id).await?;
        internal::extract_description(&document)
    }

    /// Fetches and extracts both title and description from a shindan page.
    pub async fn get_title_with_description(&self, id: &str) -> Result<(String, String)> {
        let document = self.fetch_document(id).await?;
        Ok((
            internal::extract_title(&document)?,
            internal::extract_description(&document)?,
        ))
    }

    #[cfg(feature = "segments")]
    /// Get the segments of a shindan.
    pub async fn get_segments(&self, id: &str, name: &str) -> Result<Segments> {
        let (_, response_text) = self.submit_shindan(id, name, false).await?;
        internal::parse_segments(&response_text)
    }

    #[cfg(feature = "segments")]
    /// Get the segments of a shindan and the title of the shindan.
    pub async fn get_segments_with_title(
        &self,
        id: &str,
        name: &str,
    ) -> Result<(Segments, String)> {
        let (title, response_text) = self.submit_shindan(id, name, true).await?;
        let title = title.context("Title should have been extracted")?;
        let segments = internal::parse_segments(&response_text)?;

        Ok((segments, title))
    }

    #[cfg(feature = "html")]
    /// Get the HTML string of a shindan.
    pub async fn get_html_str(&self, id: &str, name: &str) -> Result<String> {
        let (_, response_text) = self.submit_shindan(id, name, false).await?;
        internal::construct_html_result(id, &response_text, &self.domain.to_string())
    }

    #[cfg(feature = "html")]
    /// Get the HTML string of a shindan and the title of the shindan.
    pub async fn get_html_str_with_title(&self, id: &str, name: &str) -> Result<(String, String)> {
        let (title, response_text) = self.submit_shindan(id, name, true).await?;
        let title = title.context("Title should have been extracted")?;
        let html = internal::construct_html_result(id, &response_text, &self.domain.to_string())?;

        Ok((html, title))
    }

    // --- Internal Helpers ---

    async fn fetch_document(&self, id: &str) -> Result<Html> {
        let url = format!("{}{}", self.domain, id);
        let text = self.client.get(&url).send().await?.text().await?;
        Ok(Html::parse_document(&text))
    }

    /// Performs the logic of:
    /// 1. Fetching the page to get CSRF tokens and cookies (handled by jar).
    /// 2. Parsing the form data.
    /// 3. Submitting the POST request.
    /// 4. Returning optional title (if requested) and raw HTML response.
    async fn submit_shindan(
        &self,
        id: &str,
        name: &str,
        extract_title: bool,
    ) -> Result<(Option<String>, String)> {
        let url = format!("{}{}", self.domain, id);

        // 1. Initial GET to establish session and get form tokens
        let initial_response_text = self.client.get(&url).send().await?.text().await?;
        let document = Html::parse_document(&initial_response_text);

        // 2. Extract Form Data
        let form_data = internal::extract_form_data(&document, name)?;

        let title = if extract_title {
            Some(internal::extract_title(&document)?)
        } else {
            None
        };

        // 3. POST (Cookies are handled automatically by reqwest::cookie::Jar if enabled)
        // Construct headers for the POST
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // 4. Submit
        let response_text = self
            .client
            .post(&url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?
            .text()
            .await?;

        Ok((title, response_text))
    }
}

/// Domain definitions
mod domain {
    use super::*;
    use std::fmt;

    /// A domain of ShindanMaker.
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

    impl FromStr for ShindanDomain {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_uppercase().as_str() {
                "JP" => Ok(Self::Jp),
                "EN" => Ok(Self::En),
                "CN" => Ok(Self::Cn),
                "KR" => Ok(Self::Kr),
                "TH" => Ok(Self::Th),
                _ => Err(anyhow::anyhow!("Invalid domain")),
            }
        }
    }
}

/// Data models
#[cfg(feature = "segments")]
mod models {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::fmt;
    use std::ops::Deref;

    /// A segment of a shindan result.
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Segment {
        #[serde(rename = "type")]
        pub type_: String,
        pub data: Value,
    }

    impl Segment {
        pub fn new(type_: &str, data: Value) -> Self {
            Segment {
                type_: type_.to_string(),
                data,
            }
        }

        pub fn get_str(&self) -> Option<String> {
            match self.type_.as_str() {
                "text" => self
                    .data
                    .get("text")
                    .and_then(Value::as_str)
                    .map(String::from),
                "image" => self
                    .data
                    .get("file")
                    .and_then(Value::as_str)
                    .map(String::from),
                _ => None,
            }
        }
    }

    /// A collection of segments.
    #[derive(Debug, Clone)]
    pub struct Segments(pub Vec<Segment>);

    impl Deref for Segments {
        type Target = Vec<Segment>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl fmt::Display for Segments {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let str = self
                .iter()
                .filter_map(|s| s.get_str())
                .collect::<Vec<_>>()
                .join("");
            write!(f, "{}", str)
        }
    }
}

/// Internal parsing and selector logic
mod internal {
    use super::*;
    use scraper::{ElementRef, Node, Selector};
    use std::sync::OnceLock;

    static SELECTORS: OnceLock<Selectors> = OnceLock::new();

    struct Selectors {
        shindan_title: Selector,
        shindan_description: Selector,
        form_inputs: Vec<Selector>,
        input_parts: Selector,
        #[cfg(feature = "segments")]
        shindan_result: Selector,
        #[cfg(feature = "html")]
        title_and_result: Selector,
        #[cfg(feature = "html")]
        script: Selector,
        #[cfg(feature = "html")]
        effects: Vec<Selector>,
    }

    impl Selectors {
        fn get() -> &'static Self {
            SELECTORS.get_or_init(|| Self {
                shindan_title: Selector::parse("#shindanTitle").expect("Valid Selector"),
                shindan_description: Selector::parse("#shindanDescriptionDisplay")
                    .expect("Valid Selector"),
                form_inputs: vec![
                    Selector::parse("input[name=_token]").unwrap(),
                    Selector::parse("input[name=randname]").unwrap(),
                    Selector::parse("input[name=type]").unwrap(),
                ],
                input_parts: Selector::parse(r#"input[name^="parts["]"#).unwrap(),

                #[cfg(feature = "segments")]
                shindan_result: Selector::parse("#shindanResult").expect("Valid Selector"),

                #[cfg(feature = "html")]
                title_and_result: Selector::parse("#title_and_result").expect("Valid Selector"),
                #[cfg(feature = "html")]
                script: Selector::parse("script").expect("Valid Selector"),
                #[cfg(feature = "html")]
                effects: vec![
                    Selector::parse("span.shindanEffects[data-mode=ef_typing]").unwrap(),
                    Selector::parse("span.shindanEffects[data-mode=ef_shuffle]").unwrap(),
                ],
            })
        }
    }

    pub(crate) fn extract_title(dom: &Html) -> Result<String> {
        Ok(dom
            .select(&Selectors::get().shindan_title)
            .next()
            .context("Failed to find shindanTitle element")?
            .value()
            .attr("data-shindan_title")
            .context("Missing data-shindan_title attribute")?
            .to_string())
    }

    pub(crate) fn extract_description(dom: &Html) -> Result<String> {
        let mut desc = Vec::new();
        let element = dom
            .select(&Selectors::get().shindan_description)
            .next()
            .context("Failed to find description element")?;

        for child in element.children() {
            match child.value() {
                Node::Text(text) => desc.push(text.to_string()),
                Node::Element(el) if el.name() == "br" => desc.push("\n".to_string()),
                Node::Element(_) => {
                    // Handle nested text nodes (e.g., inside spans)
                    if let Some(node) = child.children().next()
                        && let Node::Text(text) = node.value()
                    {
                        desc.push(text.to_string());
                    }
                }
                _ => {}
            }
        }
        Ok(desc.join(""))
    }

    pub(crate) fn extract_form_data(dom: &Html, name: &str) -> Result<Vec<(String, String)>> {
        let selectors = Selectors::get();
        let fields = ["_token", "randname", "type"];
        let mut form_data = Vec::with_capacity(fields.len() + 2);

        for (i, &field) in fields.iter().enumerate() {
            let val = dom
                .select(&selectors.form_inputs[i])
                .next()
                .and_then(|el| el.value().attr("value"))
                .unwrap_or("")
                .to_string();
            form_data.push((field.to_string(), val));
        }

        form_data.push(("user_input_value_1".to_string(), name.to_string()));

        for el in dom.select(&selectors.input_parts) {
            if let Some(input_name) = el.value().attr("name") {
                form_data.push((input_name.to_string(), name.to_string()));
            }
        }
        Ok(form_data)
    }

    #[cfg(feature = "segments")]
    pub(crate) fn parse_segments(response_text: &str) -> Result<super::Segments> {
        use super::models::Segment;
        use serde_json::{Value, json};

        let dom = Html::parse_document(response_text);
        let mut segments = Vec::new();

        // Target the common result container
        let container_ref = dom
            .select(&Selectors::get().shindan_result)
            .next()
            .context("Failed to find shindanResult")?;

        // Strategy 1: Try parsing the `data-blocks` JSON attribute (New format).
        // This is cleaner as it contains structured data with proper newlines.
        if let Some(blocks_json) = container_ref.value().attr("data-blocks")
            && let Ok(blocks) = serde_json::from_str::<Vec<Value>>(blocks_json)
        {
            for block in blocks {
                let type_ = block["type"].as_str().unwrap_or("");
                match type_ {
                    "text" => {
                        if let Some(content) = block.get("content").and_then(|v| v.as_str()) {
                            segments.push(Segment::new("text", json!({ "text": content })));
                        }
                    }
                    "user_input" => {
                        // Treat user input as text for reading purposes
                        if let Some(val) = block.get("value").and_then(|v| v.as_str()) {
                            segments.push(Segment::new("text", json!({ "text": val })));
                        }
                    }
                    "image" => {
                        // Try to find a source URL from common keys
                        let src = block
                            .get("source")
                            .or(block.get("src"))
                            .or(block.get("url"))
                            .or(block.get("file"))
                            .and_then(|v| v.as_str());
                        if let Some(s) = src {
                            segments.push(Segment::new("image", json!({ "file": s })));
                        }
                    }
                    _ => {}
                }
            }
            // If we successfully extracted segments from JSON, return them.
            if !segments.is_empty() {
                return Ok(super::Segments(segments));
            }
        }

        // Strategy 2: Fallback to DOM traversal (Old format or JSON parse failure).
        fn extract_nodes(node: ElementRef, segments: &mut Vec<Segment>) {
            for child in node.children() {
                match child.value() {
                    Node::Text(text) => {
                        // ShindanMaker often uses &nbsp; for spacing.
                        let t = text.replace("&nbsp;", " ");
                        if !t.is_empty() {
                            segments.push(Segment::new("text", json!({ "text": t })));
                        }
                    }
                    Node::Element(el) => {
                        if el.name() == "br" {
                            segments.push(Segment::new("text", json!({ "text": "\n" })));
                        } else if el.name() == "img" {
                            let src = el.attr("data-src").or_else(|| el.attr("src"));
                            if let Some(s) = src {
                                segments.push(Segment::new("image", json!({ "file": s })));
                            }
                        } else {
                            // Recurse for wrappers like <span class="text-block"> or <span class="font-weight-bold">
                            if let Some(child_el) = ElementRef::wrap(child) {
                                extract_nodes(child_el, segments);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        extract_nodes(container_ref, &mut segments);

        Ok(super::Segments(segments))
    }

    #[cfg(feature = "html")]
    pub(crate) fn construct_html_result(
        id: &str,
        response_text: &str,
        base_url: &str,
    ) -> Result<String> {
        use anyhow::anyhow;
        use scraper::Element;

        // Note: These paths assume the original project structure.
        static APP_CSS: &str = include_str!("../static/app.css");
        static SHINDAN_JS: &str = include_str!("../static/shindan.js");
        static APP_JS: &str = include_str!("../static/app.js");
        static CHART_JS: &str = include_str!("../static/chart.js");

        let dom = Html::parse_document(response_text);
        let selectors = Selectors::get();

        let mut title_and_result = dom
            .select(&selectors.title_and_result)
            .next()
            .context("Failed to get result element")?
            .html();

        // Handle JS effects placeholders
        for selector in &selectors.effects {
            for effect in dom.select(selector) {
                if let Some(next) = effect.next_sibling_element() {
                    if next.value().name() == "noscript" {
                        title_and_result = title_and_result
                            .replace(&effect.html(), "")
                            .replace(&next.html(), &next.inner_html());
                    }
                }
            }
        }

        // Find specific script for this ID
        let mut specific_script = String::new();
        for element in dom.select(&selectors.script) {
            let html = element.html();
            if html.contains(id) {
                specific_script = html;
                break;
            }
        }
        if specific_script.is_empty() {
            return Err(anyhow!("Failed to find script with id {}", id));
        }

        let mut html = format!(
            r#"<!DOCTYPE html><html lang="zh" style="height:100%"><head><style>{}</style><meta http-equiv="Content-Type" content="text/html;charset=utf-8"><meta name="viewport" content="width=device-width,initial-scale=1.0,minimum-scale=1.0"><base href="{}"><title>ShindanMaker</title></head><body class="" style="position:relative;min-height:100%;top:0"><div id="main-container"><div id="main">{}</div></div></body><script>{}</script><!-- SCRIPTS --></html>"#,
            APP_CSS, base_url, title_and_result, SHINDAN_JS
        );

        if response_text.contains("chart.js") {
            let scripts = format!(
                "<script>{}</script>\n<script>{}</script>\n{}",
                APP_JS, CHART_JS, specific_script
            );
            html = html.replace("<!-- SCRIPTS -->", &scripts);
        }

        Ok(html)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_title() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();
        let (title, _) = client.get_title_with_description("1222992").await.unwrap();
        assert_eq!("Fantasy Stats", title);
    }
}
