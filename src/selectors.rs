use scraper::Selector;
use once_cell::sync::Lazy;

pub(crate) static SELECTORS: Lazy<Selectors> = Lazy::new(Selectors::new);

#[derive(Clone)]
pub(crate) struct Selectors {
    pub(crate) shindan_title: Selector,
    pub(crate) shindan_description_display: Selector,
    pub(crate) form: Vec<Selector>,

    #[cfg(feature = "segments")]
    pub(crate) post_display: Selector,

    #[cfg(feature = "html")]
    pub(crate) title_and_result: Selector,
    #[cfg(feature = "html")]
    pub(crate) script: Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
            shindan_title: Selector::parse("#shindanTitle").expect("Failed to parse selector"),
            shindan_description_display: Selector::parse("#shindanDescriptionDisplay").expect("Failed to parse selector"),
            form: vec![
                Selector::parse("input[name=_token]").expect("Failed to parse selector"),
                Selector::parse("input[name=randname]").expect("Failed to parse selector"),
                Selector::parse("input[name=type]").expect("Failed to parse selector"),
            ],

            #[cfg(feature = "segments")]
            post_display: Selector::parse("#post_display").expect("Invalid selector"),

            #[cfg(feature = "html")]
            title_and_result: Selector::parse("#title_and_result").expect("Failed to parse selector"),
            #[cfg(feature = "html")]
            script: Selector::parse("script").expect("Invalid script selector"),
        }
    }
}