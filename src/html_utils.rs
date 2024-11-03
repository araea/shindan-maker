use serde_json::json;
use scraper::{Html, Node};
use anyhow::{Context, Result};

use crate::selectors::SELECTORS;

#[cfg(feature = "segments")]
use crate::segment::{Segment, Segments};

#[cfg(feature = "html")]
use {
    anyhow::anyhow,
    scraper::Element,
    crate::html_template::HTML_TEMPLATE,
};

#[cfg(feature = "segments")]
pub(crate) fn get_segments(response_text: &str) -> Result<Segments> {
    let result_document = Html::parse_document(response_text);

    let mut segments = Vec::new();

    result_document.select(&SELECTORS.post_display)
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

    Ok(Segments(segments))
}

#[cfg(feature = "html")]
pub(crate) fn get_html_str(id: &str, response_text: &str) -> Result<String> {
    let result_document = Html::parse_document(response_text);

    let mut title_and_result = result_document
        .select(&SELECTORS.title_and_result)
        .next()
        .context("Failed to get the next element")?
        .html();

    for effects_selector in &SELECTORS.effects {
        let effects = result_document.select(effects_selector);
        for effect in effects {
            if let Some(next_el) = effect.next_sibling_element() {
                if next_el.value().name() == "noscript" {
                    let content = next_el.inner_html();

                    title_and_result = title_and_result.replace(&effect.html(), "")
                        .replace(&next_el.html(), &content);
                }
            }
        }
    }

    let mut html = HTML_TEMPLATE
        .replace("<!-- TITLE_AND_RESULT -->", &title_and_result);

    if response_text.contains("chart.js") {
        let mut scripts = vec![
            r#"<script src="https://cn.shindanmaker.com/js/app.js?id=163959a7e23bfa7264a0ddefb3c36f13" defer=""></script>"#,
            r#"<script src="https://cn.shindanmaker.com/js/chart.js?id=391e335afc72362acd6bf1ea1ba6b74c" defer=""></script>"#];

        let shindan_script = get_first_script(&result_document, id)?;
        scripts.push(&shindan_script);
        html = html.replace("<!-- SCRIPTS -->", &scripts.join("\n"));
    }
    Ok(html)
}

#[cfg(feature = "html")]
pub(crate) fn get_first_script(result_document: &Html, id: &str) -> Result<String> {
    for element in result_document.select(&SELECTORS.script) {
        let html = element.html();
        if html.contains(id) {
            return Ok(html);
        }
    }

    Err(anyhow!("Failed to find script with id {}", id))
}

pub(crate) fn extract_title_and_form_data(html_content: &str, name: &str) -> Result<(String, Vec<(&'static str, String)>)> {
    let document = Html::parse_document(html_content);
    let title = extract_title(&document)?;
    let form_data = extract_form_data(&document, name)?;

    Ok((title, form_data))
}

pub(crate) fn extract_title(dom: &Html) -> Result<String> {
    Ok(dom
        .select(&SELECTORS.shindan_title)
        .next()
        .context("Failed to get the next element")?
        .value().attr("data-shindan_title")
        .context("Failed to get 'data-shindan_title' attribute")?
        .to_string())
}

pub(crate) fn extract_description(dom: &Html) -> Result<String> {
    let mut desc = Vec::new();

    dom
        .select(&SELECTORS.shindan_description_display)
        .next()
        .context("Failed to get the next element")?
        .children()
        .for_each(|child| {
            let node = child.value();
            match node {
                Node::Text(text) => {
                    desc.push(text.to_string());
                }
                Node::Element(element) => {
                    if element.name() == "br" {
                        desc.push("\n".to_string());
                    } else if let Some(node) = child.children().next() {
                        if let Node::Text(text) = node.value() {
                            desc.push(text.to_string());
                        };
                    }
                }
                _ => {}
            }
        });

    Ok(desc.join(""))
}

pub(crate) fn extract_form_data(
    dom: &Html,
    name: &str,
) -> Result<Vec<(&'static str, String)>> {
    const FIELDS: &[&str] = &["_token", "randname", "type"];
    let mut form_data = Vec::with_capacity(FIELDS.len() + 1);

    for (index, &field) in FIELDS.iter().enumerate() {
        let value = dom
            .select(&SELECTORS.form[index])
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