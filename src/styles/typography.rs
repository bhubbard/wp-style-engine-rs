use crate::types::{Style, StyleOptions, GeneratedCSSRule};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(typo) = &style.typography {
        // 1. fontFamily
        if let Some(family) = &typo.font_family {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "fontFamily".to_string(),
                value: get_css_value_from_raw_style(family),
            });
        }

        // 2. fontSize
        if let Some(size) = &typo.font_size {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "fontSize".to_string(),
                value: get_css_value_from_raw_style(size),
            });
        }

        // 3. fontStyle
        if let Some(style_val) = &typo.font_style {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "fontStyle".to_string(),
                value: get_css_value_from_raw_style(style_val),
            });
        }

        // 4. fontWeight
        if let Some(weight) = &typo.font_weight {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "fontWeight".to_string(),
                value: get_css_value_from_raw_style(weight),
            });
        }

        // 5. letterSpacing
        if let Some(spacing) = &typo.letter_spacing {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "letterSpacing".to_string(),
                value: get_css_value_from_raw_style(spacing),
            });
        }

        // 6. lineHeight
        if let Some(lh) = &typo.line_height {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "lineHeight".to_string(),
                value: get_css_value_from_raw_style(lh),
            });
        }

        // 7. textColumns -> columnCount
        if let Some(cols) = &typo.text_columns {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "columnCount".to_string(),
                value: get_css_value_from_raw_style(cols),
            });
        }

        // 8. textDecoration
        if let Some(dec) = &typo.text_decoration {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "textDecoration".to_string(),
                value: get_css_value_from_raw_style(dec),
            });
        }

        // 9. textIndent
        if let Some(indent) = &typo.text_indent {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "textIndent".to_string(),
                value: get_css_value_from_raw_style(indent),
            });
        }

        // 10. textTransform
        if let Some(transform) = &typo.text_transform {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "textTransform".to_string(),
                value: get_css_value_from_raw_style(transform),
            });
        }

        // 11. writingMode
        if let Some(mode) = &typo.writing_mode {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "writingMode".to_string(),
                value: get_css_value_from_raw_style(mode),
            });
        }
    }

    rules
}
