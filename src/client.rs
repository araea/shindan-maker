use crate::domain::ShindanDomain;
use crate::internal;
use anyhow::{Context, Result};
use reqwest::{Client, header};
use scraper::Html;
use std::time::Duration;

#[cfg(feature = "segments")]
use crate::models::Segments;

/// A client for interacting with ShindanMaker.
#[derive(Clone, Debug)]
pub struct ShindanClient {
    client: Client,
    domain: ShindanDomain,
}

impl ShindanClient {
    /// Create a new ShindanMaker client.
    pub fn new(domain: ShindanDomain) -> Result<Self> {
        const TIMEOUT_SECS: u64 = 30;
        const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

        Ok(Self {
            domain,
            client: Client::builder()
                .user_agent(USER_AGENT)
                .use_rustls_tls()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .cookie_store(true)
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

    async fn submit_shindan(
        &self,
        id: &str,
        name: &str,
        extract_title: bool,
    ) -> Result<(Option<String>, String)> {
        let url = format!("{}{}", self.domain, id);

        // 1. Initial GET
        let initial_response = self.client.get(&url).send().await?;
        let initial_response_text = initial_response.text().await?;

        let document = Html::parse_document(&initial_response_text);

        // 2. Extract Form Data
        let form_data = internal::extract_form_data(&document, name)?;

        let title = if extract_title {
            Some(internal::extract_title(&document)?)
        } else {
            None
        };

        // 3. POST
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let post_response = self
            .client
            .post(&url)
            .headers(headers)
            .form(&form_data)
            .send()
            .await?;
        let response_text = post_response.text().await?;

        Ok((title, response_text))
    }
}
