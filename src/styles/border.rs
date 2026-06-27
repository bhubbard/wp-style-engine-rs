use crate::types::{Style, StyleOptions, GeneratedCSSRule, BorderRadius, BorderIndividualStyles};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(border) = &style.border {
        // 1. color
        if let Some(color) = &border.color {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "borderColor".to_string(),
                value: get_css_value_from_raw_style(color),
            });
        }

        // 2. style
        if let Some(border_style) = &border.style {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "borderStyle".to_string(),
                value: get_css_value_from_raw_style(border_style),
            });
        }

        // 3. width
        if let Some(width) = &border.width {
            rules.push(GeneratedCSSRule {
                selector: options.selector.clone(),
                key: "borderWidth".to_string(),
                value: get_css_value_from_raw_style(width),
            });
        }

        // 4. radius
        if let Some(radius) = &border.radius {
            match radius {
                BorderRadius::Value(v) => {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: "borderRadius".to_string(),
                        value: get_css_value_from_raw_style(v),
                    });
                }
                BorderRadius::Object { top_left, top_right, bottom_left, bottom_right } => {
                    if let Some(tl) = top_left {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "borderTopLeftRadius".to_string(),
                            value: get_css_value_from_raw_style(tl),
                        });
                    }
                    if let Some(tr) = top_right {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "borderTopRightRadius".to_string(),
                            value: get_css_value_from_raw_style(tr),
                        });
                    }
                    if let Some(bl) = bottom_left {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "borderBottomLeftRadius".to_string(),
                            value: get_css_value_from_raw_style(bl),
                        });
                    }
                    if let Some(br) = bottom_right {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "borderBottomRightRadius".to_string(),
                            value: get_css_value_from_raw_style(br),
                        });
                    }
                }
            }
        }

        // Helper to push individual edge styles
        let mut push_edge = |edge_styles: &Option<BorderIndividualStyles>, edge_name: &str| {
            if let Some(edge) = edge_styles {
                if let Some(color) = &edge.color {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: format!("border{}Color", edge_name),
                        value: get_css_value_from_raw_style(color),
                    });
                }
                if let Some(style) = &edge.style {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: format!("border{}Style", edge_name),
                        value: get_css_value_from_raw_style(style),
                    });
                }
                if let Some(width) = &edge.width {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: format!("border{}Width", edge_name),
                        value: get_css_value_from_raw_style(width),
                    });
                }
            }
        };

        // 5. borderTop, borderRight, borderBottom, borderLeft
        push_edge(&border.top, "Top");
        push_edge(&border.right, "Right");
        push_edge(&border.bottom, "Bottom");
        push_edge(&border.left, "Left");
    }

    rules
}
