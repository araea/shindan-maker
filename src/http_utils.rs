use reqwest::header;
use anyhow::{Context, Result};

pub(crate) fn extract_session_cookie(response: &reqwest::Response) -> Result<String> {
    response.cookies()
        .find(|cookie| cookie.name() == "_session")
        .map(|cookie| cookie.value().to_string())
        .context("Failed to extract session cookie")
}

pub(crate) fn prepare_headers(session_cookie: &str) -> Result<header::HeaderMap> {
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