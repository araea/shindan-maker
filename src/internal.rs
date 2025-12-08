use anyhow::{Context, Result};
use scraper::{Html, Node, Selector};
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
pub(crate) fn parse_segments(response_text: &str) -> Result<crate::models::Segments> {
    use crate::models::{Segment, Segments};
    use scraper::ElementRef;
    use serde_json::{Value, json};

    let dom = Html::parse_document(response_text);
    let mut segments = Vec::new();

    let container_ref = dom
        .select(&Selectors::get().shindan_result)
        .next()
        .context("Failed to find shindanResult")?;

    // Strategy 1: Try parsing the `data-blocks` JSON attribute
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
                    if let Some(val) = block.get("value").and_then(|v| v.as_str()) {
                        segments.push(Segment::new("text", json!({ "text": val })));
                    }
                }
                "image" => {
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
        if !segments.is_empty() {
            return Ok(Segments(segments));
        }
    }

    // Strategy 2: Fallback to DOM traversal
    fn extract_nodes(node: ElementRef, segments: &mut Vec<Segment>) {
        for child in node.children() {
            match child.value() {
                Node::Text(text) => {
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
                    } else if let Some(child_el) = ElementRef::wrap(child) {
                        extract_nodes(child_el, segments);
                    }
                }
                _ => {}
            }
        }
    }

    extract_nodes(container_ref, &mut segments);

    Ok(Segments(segments))
}

#[cfg(feature = "html")]
pub(crate) fn construct_html_result(
    id: &str,
    response_text: &str,
    base_url: &str,
) -> Result<String> {
    use anyhow::anyhow;
    use scraper::Element;

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
