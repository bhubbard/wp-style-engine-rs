use crate::types::{Style, StyleOptions, GeneratedCSSRule};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(dims) = &style.dimensions {
        // 1. height
        if let Some(height) = &dims.height {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "height".to_string(),
                value: get_css_value_from_raw_style(height),
            });
        }

        // 2. minHeight
        if let Some(min_height) = &dims.min_height {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "minHeight".to_string(),
                value: get_css_value_from_raw_style(min_height),
            });
        }

        // 3. minWidth
        if let Some(min_width) = &dims.min_width {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "minWidth".to_string(),
                value: get_css_value_from_raw_style(min_width),
            });
        }

        // 4. aspectRatio
        if let Some(aspect_ratio) = &dims.aspect_ratio {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "aspectRatio".to_string(),
                value: get_css_value_from_raw_style(aspect_ratio),
            });
        }

        // 5. width
        if let Some(width) = &dims.width {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "width".to_string(),
                value: get_css_value_from_raw_style(width),
            });
        }
    }

    rules
}
