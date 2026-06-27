pub mod background;
pub mod border;
pub mod color;
pub mod dimensions;
pub mod outline;
pub mod spacing;
pub mod typography;

use crate::types::{Style, StyleOptions, GeneratedCSSRule};
use crate::utils::get_css_value_from_raw_style;

/// Aggregates generated CSS rules in the exact order defined by the JavaScript Style Engine.
pub fn get_rules(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    // 1. border
    rules.extend(border::generate(style, options));
    
    // 2. color
    rules.extend(color::generate(style, options));

    // 3. dimensions
    rules.extend(dimensions::generate(style, options));

    // 4. outline
    rules.extend(outline::generate(style, options));

    // 5. spacing
    rules.extend(spacing::generate(style, options));

    // 6. typography
    rules.extend(typography::generate(style, options));

    // 7. shadow (boxShadow)
    if let Some(shadow) = &style.shadow {
        rules.push(GeneratedCSSRule {
            selector: options.selector.clone(),
            key: "boxShadow".to_string(),
            value: get_css_value_from_raw_style(shadow),
        });
    }

    // 8. background
    rules.extend(background::generate(style, options));

    rules
}
