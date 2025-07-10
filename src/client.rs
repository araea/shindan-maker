use anyhow::Result;
use reqwest::Client;
use scraper::Html;
use std::time::Duration;

use crate::html_utils;
use crate::http_utils;
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
                .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/140.0.0.0 Safari/537.36 Edg/140.0.0.0")
                .use_rustls_tls()
                .timeout(Duration::from_secs(TIMEOUT_SECS))
                .build()?,
        })
    }

    /**
    Fetches and extracts title from a shindan page.

    # Arguments
    - `id` - The ID of the shindan

    # Returns
    The title of the shindan page.

    # Errors
    Returns error if network request fails or title cannot be extracted.

    # Examples
    ```
    use anyhow::Result;
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() -> Result<()> {
        let client = ShindanClient::new(ShindanDomain::En)?;

        let title = client
            .get_title("1222992")
            .await?;

        println!("Title: {}", title);

        Ok(())
    }
    ```
    */
    pub async fn get_title(&self, id: &str) -> Result<String> {
        let document = self.fetch_document(id).await?;
        html_utils::extract_title(&document)
    }

    /**
    Fetches and extracts description from a shindan page.

    # Arguments
    - `id` - The ID of the shindan

    # Returns
    The description of the shindan page.

    # Errors
    Returns error if network request fails or description cannot be extracted.

    # Examples
    ```
    use anyhow::Result;
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() -> Result<()> {
        let client = ShindanClient::new(ShindanDomain::En)?;

        let desc = client
            .get_description("1222992")
            .await?;

        println!("Description: {}", desc);

        Ok(())
    }
    ```
    */
    pub async fn get_description(&self, id: &str) -> Result<String> {
        let document = self.fetch_document(id).await?;
        html_utils::extract_description(&document)
    }

    /**
    Fetches and extracts both title and description from a shindan page.

    # Arguments
    - `id` - The ID of the shindan

    # Returns
    A tuple containing the title and description.

    # Errors
    Returns error if network request fails or content cannot be extracted.

    # Examples
    ```
    use anyhow::Result;
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() -> Result<()> {
        let client = ShindanClient::new(ShindanDomain::En)?;

        let (title, desc) = client
            .get_title_with_description("1222992")
            .await?;

        println!("Title: {}", title);
        println!("Description: {}", desc);

        Ok(())
    }
    ```
    */
    pub async fn get_title_with_description(&self, id: &str) -> Result<(String, String)> {
        let document = self.fetch_document(id).await?;

        Ok((
            html_utils::extract_title(&document)?,
            html_utils::extract_description(&document)?,
        ))
    }

    async fn fetch_document(&self, id: &str) -> Result<Html> {
        let url = format!("{}{}", self.domain, id);

        let text = self.client.get(&url).send().await?.text().await?;

        Ok(Html::parse_document(&text))
    }

    async fn fetch_with_form_data(
        &self,
        id: &str,
        name: &str,
        extract_title: bool,
    ) -> Result<(Option<String>, String)> {
        let url = format!("{}{}", self.domain, id);

        let initial_response = self.client.get(&url).send().await?;
        let session_cookie = http_utils::extract_session_cookie(&initial_response)?;
        let initial_response_text = initial_response.text().await?;

        let (title, form_data) = if extract_title {
            let (title, form_data) =
                html_utils::extract_title_and_form_data(&initial_response_text, name)?;
            (Some(title), form_data)
        } else {
            let document = Html::parse_document(&initial_response_text);
            let form_data = html_utils::extract_form_data(&document, name)?;
            (None, form_data)
        };

        let headers = http_utils::prepare_headers(&session_cookie)?;
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

    async fn init_res(&self, id: &str, name: &str) -> Result<String> {
        let (_, response_text) = self.fetch_with_form_data(id, name, false).await?;
        Ok(response_text)
    }

    async fn get_title_and_init_res(&self, id: &str, name: &str) -> Result<(String, String)> {
        let (title, response_text) = self.fetch_with_form_data(id, name, true).await?;
        Ok((title.unwrap(), response_text))
    }

    /**
    Get the segments of a shindan.

    # Arguments
    - `id` - The ID of the shindan.
    - `name` - The name to use for the shindan.

    # Returns
    The segments of the shindan.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let segments = client
            .get_segments("1222992", "test_user")
            .await
            .unwrap();

        println!("Result segments: {:#?}", segments);
    }
    ```
    */
    #[cfg(feature = "segments")]
    pub async fn get_segments(&self, id: &str, name: &str) -> Result<Segments> {
        let response_text = self.init_res(id, name).await?;
        html_utils::get_segments(&response_text)
    }

    /**
    Get the segments of a shindan and the title of the shindan.

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
    The HTML string of the shindan.

    # Examples
    ```
    use shindan_maker::{ShindanClient, ShindanDomain};

    #[tokio::main]
    async fn main() {
        let client = ShindanClient::new(ShindanDomain::En).unwrap();

        let html_str = client
            .get_html_str("1222992", "test_user")
            .await
            .unwrap();

        println!("{}", html_str);
    }
    ```
    */
    #[cfg(feature = "html")]
    pub async fn get_html_str(&self, id: &str, name: &str) -> Result<String> {
        let response_text = self.init_res(id, name).await?;
        html_utils::get_html_str(id, &response_text)
    }

    /**
    Get the HTML string of a shindan and the title of the shindan.

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
    pub async fn get_html_str_with_title(&self, id: &str, name: &str) -> Result<(String, String)> {
        let (title, response_text) = self.get_title_and_init_res(id, name).await?;

        let html = html_utils::get_html_str(id, &response_text)?;

        Ok((html, title))
    }
}
