use anyhow::{Context, Result};
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderValue};

pub(crate) fn extract_session_cookie(response: &reqwest::Response) -> Result<String> {
    response
        .cookies()
        .find(|cookie| cookie.name() == "_session")
        .map(|cookie| cookie.value().to_string())
        .context("Failed to extract session cookie")
}

pub(crate) fn prepare_headers(session_cookie: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    let cookie_value = format!("_session={};", session_cookie);
    headers.insert(header::COOKIE, HeaderValue::from_str(&cookie_value)?);

    Ok(headers)
}
