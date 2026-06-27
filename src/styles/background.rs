use crate::types::{Style, StyleOptions, GeneratedCSSRule, BackgroundImage};
use crate::utils::{get_css_value_from_raw_style, safe_decode_uri, encode_uri};

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(background) = &style.background {
        // 1. backgroundImage
        let gradient = background.gradient.as_ref()
            .map(|g| get_css_value_from_raw_style(g))
            .unwrap_or_default();
        
        let bg_image_val = match &background.background_image {
            Some(BackgroundImage::Object { url, .. }) => {
                url.as_ref()
                    .map(|u| format!("url( '{}' )", encode_uri(&safe_decode_uri(u))))
                    .unwrap_or_default()
            }
            Some(BackgroundImage::Value(v)) => get_css_value_from_raw_style(v),
            None => String::new(),
        };

        let css_value = if !gradient.is_empty() && !bg_image_val.is_empty() {
            format!("{}, {}", gradient, bg_image_val)
        } else if !gradient.is_empty() {
            gradient
        } else {
            bg_image_val
        };

        if !css_value.is_empty() {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundImage".to_string(),
                value: css_value,
            });
        }

        // 2. backgroundPosition
        if let Some(pos) = &background.background_position {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundPosition".to_string(),
                value: get_css_value_from_raw_style(pos),
            });
        }

        // 3. backgroundRepeat
        if let Some(rep) = &background.background_repeat {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundRepeat".to_string(),
                value: get_css_value_from_raw_style(rep),
            });
        }

        // 4. backgroundSize
        if let Some(size) = &background.background_size {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundSize".to_string(),
                value: get_css_value_from_raw_style(size),
            });
        }

        // 5. backgroundAttachment
        if let Some(att) = &background.background_attachment {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "backgroundAttachment".to_string(),
                value: get_css_value_from_raw_style(att),
            });
        }
    }

    rules
}
