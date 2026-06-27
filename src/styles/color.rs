use crate::types::{Style, StyleOptions, GeneratedCSSRule};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(color) = &style.color {
        // 1. text -> color
        if let Some(text) = &color.text {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "color".to_string(),
                value: get_css_value_from_raw_style(text),
            });
        }

        // 2. gradient -> background
        if let Some(gradient) = &color.gradient {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "background".to_string(),
                value: get_css_value_from_raw_style(gradient),
            });
        }

        // 3. background -> backgroundColor
        if let Some(bg) = &color.background {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundColor".to_string(),
                value: get_css_value_from_raw_style(bg),
            });
        }
    }

    rules
}
