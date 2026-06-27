use crate::types::{Style, StyleOptions, GeneratedCSSRule, ValueOrBox};
use crate::utils::get_css_value_from_raw_style;

pub fn generate(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    let mut rules = Vec::new();

    if let Some(spacing) = &style.spacing {
        // 1. margin
        if let Some(margin) = &spacing.margin {
            match margin {
                ValueOrBox::Value(v) => {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: "margin".to_string(),
                        value: get_css_value_from_raw_style(v),
                    });
                }
                ValueOrBox::Box(box_model) => {
                    if let Some(top) = &box_model.top {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "marginTop".to_string(),
                            value: get_css_value_from_raw_style(top),
                        });
                    }
                    if let Some(right) = &box_model.right {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "marginRight".to_string(),
                            value: get_css_value_from_raw_style(right),
                        });
                    }
                    if let Some(bottom) = &box_model.bottom {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "marginBottom".to_string(),
                            value: get_css_value_from_raw_style(bottom),
                        });
                    }
                    if let Some(left) = &box_model.left {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "marginLeft".to_string(),
                            value: get_css_value_from_raw_style(left),
                        });
                    }
                }
            }
        }

        // 2. padding
        if let Some(padding) = &spacing.padding {
            match padding {
                ValueOrBox::Value(v) => {
                    rules.push(GeneratedCSSRule {
                        selector: options.selector.clone(),
                        key: "padding".to_string(),
                        value: get_css_value_from_raw_style(v),
                    });
                }
                ValueOrBox::Box(box_model) => {
                    if let Some(top) = &box_model.top {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "paddingTop".to_string(),
                            value: get_css_value_from_raw_style(top),
                        });
                    }
                    if let Some(right) = &box_model.right {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "paddingRight".to_string(),
                            value: get_css_value_from_raw_style(right),
                        });
                    }
                    if let Some(bottom) = &box_model.bottom {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "paddingBottom".to_string(),
                            value: get_css_value_from_raw_style(bottom),
                        });
                    }
                    if let Some(left) = &box_model.left {
                        rules.push(GeneratedCSSRule {
                            selector: options.selector.clone(),
                            key: "paddingLeft".to_string(),
                            value: get_css_value_from_raw_style(left),
                        });
                    }
                }
            }
        }
    }

    rules
}
