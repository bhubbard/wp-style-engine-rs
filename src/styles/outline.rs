use crate::types::{Style, StyleOptions, GeneratedCSSRule};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(outline) = &style.outline {
        // 1. color
        if let Some(color) = &outline.color {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "outlineColor".to_string(),
                value: get_css_value_from_raw_style(color),
            });
        }

        // 2. style
        if let Some(border_style) = &outline.style {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "outlineStyle".to_string(),
                value: get_css_value_from_raw_style(border_style),
            });
        }

        // 3. offset
        if let Some(offset) = &outline.offset {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "outlineOffset".to_string(),
                value: get_css_value_from_raw_style(offset),
            });
        }

        // 4. width
        if let Some(width) = &outline.width {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "outlineWidth".to_string(),
                value: get_css_value_from_raw_style(width),
            });
        }
    }

    rules
}
