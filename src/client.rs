use scraper::Html;
use reqwest::Client;
use anyhow::Result;
use std::time::Duration;

use crate::http_utils;
use crate::html_utils;
use crate::shindan_domain::ShindanDomain;

#[cfg(feature = "segments")]
use crate::segment::Segments;

/// A client for interacting with ShindanMaker.
#[derive(Clone, Debug)]
pub struct ShindanClient {
    client: Client,
    domain: ShindanDomain,
}

impl ShindanClient {
    /**
    Create a new ShindanMaker client.

    # Arguments
    - `domain` - The domain of ShindanMaker to use.

    # Returns
    A new ShindanMaker client.

    # Examples
    ```
    use anyhow::Result;
    use shindan_maker::{ShindanClient, ShindanDomain};

    fn main() -> Result<()> {
        let client = ShindanClient::new(ShindanDomain::En)?; // Enum variant
        let client = ShindanClient::new("Jp".parse()?)?; // String slice
        let client = ShindanClient::new("EN".parse()?)?; // Case-insensitive
        let client = ShindanClient::new(String::from("cn").parse()?)?; // String
        Ok(())
    }
    ```
    */
    pub fn new(domain: ShindanDomain) -> Result<Self> {
        const TIMEOUT_SECS: u64 = 3;

        Ok(Self {
            domain,
            client: Client::builder()
                .user_agent("shindan-maker")
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()?,
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

        let title = client
            .get_title("1222992")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
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
        html_utils::extract_title(&document)
    }

    async fn get_title_and_init_res(&self, id: &str, name: &str) -> Result<(String, String)> {
        let url = format!("{}{}", self.domain, id);
        let initial_response = self.client
            .get(&url)
            .send()
            .await?;

        let session_cookie = http_utils::extract_session_cookie(&initial_response)?;
        let initial_response_text = initial_response
            .text()
            .await?;

        let (title, form_data) = html_utils::extract_title_and_form_data(&initial_response_text, name)?;

        let headers = http_utils::prepare_headers(&session_cookie)?;
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

        let (segments, title) = client
            .get_segments_with_title("1222992", "test_user")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);

        println!("Result title: {}", title);
        println!("Result text: {}", segments);

        println!("Result segments: {:#?}", segments);
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

        let segments = html_utils::get_segments(&response_text)?;

        Ok((segments, title))
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
        
        let (_html_str, title) = client
            .get_html_str_with_title("1222992", "test_user")
            .await
            .unwrap();

        assert_eq!("Fantasy Stats", title);
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

        let html = html_utils::get_html_str(id, &response_text)?;

        Ok((html, title))
    }
}