pub mod types;
pub mod utils;
pub mod styles;

pub use types::{Style, StyleOptions, GeneratedCSSRule};
pub use utils::get_css_value_from_raw_style;

use utils::kebab_case;

/// Generates a stylesheet (with braces and selectors) or inline styles (semicolon-separated) for a given style object.
pub fn compile_css(style: &Style, options: &StyleOptions) -> String {
    let rules = get_css_rules(style, options);

    if options.selector.is_none() {
        let inline_rules: Vec<String> = rules
            .iter()
            .map(|rule| format!("{}: {};", kebab_case(&rule.key), rule.value))
            .collect();
        return inline_rules.join(" ");
    }

    // Group rules by selector, preserving insertion order
    use std::collections::HashMap;
    let mut ordered_selectors = Vec::new();
    let mut grouped_rules: HashMap<String, Vec<&GeneratedCSSRule>> = HashMap::new();

    for rule in &rules {
        if let Some(selector) = &rule.selector {
            if !grouped_rules.contains_key(selector) {
                ordered_selectors.push(selector.clone());
            }
            grouped_rules.entry(selector.clone()).or_default().push(rule);
        }
    }

    let mut selector_rules = Vec::new();
    for selector in ordered_selectors {
        if let Some(rules_group) = grouped_rules.get(&selector) {
            let declarations: Vec<String> = rules_group
                .iter()
                .map(|rule| format!("{}: {};", kebab_case(&rule.key), rule.value))
                .collect();
            selector_rules.push(format!("{} {{ {} }}", selector, declarations.join(" ")));
        }
    }

    selector_rules.join("\n")
}

/// Returns a list of individual GeneratedCSSRule objects.
pub fn get_css_rules(style: &Style, options: &StyleOptions) -> Vec<GeneratedCSSRule> {
    styles::get_rules(style, options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn should_generate_empty_style() {
        let style: Style = serde_json::from_str("{}").unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        assert_eq!(compile_css(&style, &options), "");
    }

    #[test]
    fn should_generate_empty_style_with_empty_keys() {
        let style: Style = serde_json::from_str(r##"{
            "spacing": null,
            "color": null
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(compile_css(&style, &options), "");
    }

    #[test]
    fn should_generate_inline_styles_where_there_is_no_selector() {
        let style: Style = serde_json::from_str(r##"{
            "spacing": { "padding": "10px", "margin": "12px" },
            "color": {
                "text": "#f1f1f1",
                "background": "#222222",
                "gradient": "linear-gradient(135deg,rgb(6,147,227) 0%,rgb(143,47,47) 49%,rgb(155,81,224) 100%)"
            }
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(
            compile_css(&style, &options),
            "color: #f1f1f1; background: linear-gradient(135deg,rgb(6,147,227) 0%,rgb(143,47,47) 49%,rgb(155,81,224) 100%); background-color: #222222; margin: 12px; padding: 10px;"
        );
    }

    #[test]
    fn should_generate_styles_with_an_optional_selector() {
        let style1: Style = serde_json::from_str(r##"{
            "spacing": { "padding": "10px", "margin": "12px" }
        }"##).unwrap();
        let options1 = StyleOptions { selector: Some(".some-selector".to_string()) };
        assert_eq!(
            compile_css(&style1, &options1),
            ".some-selector { margin: 12px; padding: 10px; }"
        );

        let style2: Style = serde_json::from_str(r##"{
            "color": {
                "text": "#cccccc",
                "background": "#111111",
                "gradient": "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%)"
            },
            "dimensions": {
                "minHeight": "50vh",
                "minWidth": "25vw"
            },
            "spacing": {
                "padding": { "top": "10px", "bottom": "5px" },
                "margin": {
                    "top": "11px",
                    "right": "12px",
                    "bottom": "13px",
                    "left": "14px"
                }
            },
            "typography": {
                "fontSize": "2.2rem",
                "fontStyle": "italic",
                "fontWeight": "800",
                "fontFamily": "'Helvetica Neue',sans-serif",
                "lineHeight": "3.3",
                "textColumns": "2",
                "textDecoration": "line-through",
                "letterSpacing": "12px",
                "textTransform": "uppercase"
            },
            "outline": {
                "offset": "2px",
                "width": "4px",
                "style": "dashed",
                "color": "red"
            }
        }"##).unwrap();
        let options2 = StyleOptions { selector: Some(".some-selector".to_string()) };
        assert_eq!(
            compile_css(&style2, &options2),
            ".some-selector { color: #cccccc; background: linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%); background-color: #111111; min-height: 50vh; min-width: 25vw; outline-color: red; outline-style: dashed; outline-offset: 2px; outline-width: 4px; margin-top: 11px; margin-right: 12px; margin-bottom: 13px; margin-left: 14px; padding-top: 10px; padding-bottom: 5px; font-family: 'Helvetica Neue',sans-serif; font-size: 2.2rem; font-style: italic; font-weight: 800; letter-spacing: 12px; line-height: 3.3; column-count: 2; text-decoration: line-through; text-transform: uppercase; }"
        );
    }

    #[test]
    fn should_parse_preset_values() {
        let style: Style = serde_json::from_str(r##"{
            "color": {
                "text": "var:preset|color|ham-sandwich"
            },
            "spacing": { "margin": "3px" }
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(
            compile_css(&style, &options),
            "color: var(--wp--preset--color--ham-sandwich); margin: 3px;"
        );
    }

    #[test]
    fn should_parse_preset_values_and_kebab_case_the_slug() {
        let style: Style = serde_json::from_str(r##"{
            "color": {
                "text": "var:preset|font-size|h1"
            },
            "spacing": {
                "margin": { "top": "var:preset|spacing|3XL" }
            }
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(
            compile_css(&style, &options),
            "color: var(--wp--preset--font-size--h-1); margin-top: var(--wp--preset--spacing--3-xl);"
        );
    }

    #[test]
    fn should_parse_border_rules() {
        let style: Style = serde_json::from_str(r##"{
            "border": {
                "color": "var:preset|color|perky-peppermint",
                "width": "0.5em",
                "style": "dotted",
                "radius": "11px"
            }
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(
            compile_css(&style, &options),
            "border-color: var(--wp--preset--color--perky-peppermint); border-style: dotted; border-width: 0.5em; border-radius: 11px;"
        );
    }

    #[test]
    fn should_parse_individual_border_rules() {
        let style: Style = serde_json::from_str(r##"{
            "border": {
                "top": {
                    "color": "var:preset|color|sandy-beach",
                    "width": "9px",
                    "style": "dashed"
                },
                "right": {
                    "color": "var:preset|color|leafy-avenue",
                    "width": "5rem"
                },
                "bottom": {
                    "color": "#eee",
                    "width": "2%",
                    "style": "solid"
                },
                "left": {
                    "color": "var:preset|color|avocado-blues",
                    "width": "100px",
                    "style": "dotted"
                },
                "radius": {
                    "topLeft": "1px",
                    "topRight": "2px",
                    "bottomLeft": "3px",
                    "bottomRight": "4px"
                }
            }
        }"##).unwrap();
        let options = StyleOptions::default();
        assert_eq!(
            compile_css(&style, &options),
            "border-top-left-radius: 1px; border-top-right-radius: 2px; border-bottom-left-radius: 3px; border-bottom-right-radius: 4px; border-top-color: var(--wp--preset--color--sandy-beach); border-top-style: dashed; border-top-width: 9px; border-right-color: var(--wp--preset--color--leafy-avenue); border-right-width: 5rem; border-bottom-color: #eee; border-bottom-style: solid; border-bottom-width: 2%; border-left-color: var(--wp--preset--color--avocado-blues); border-left-style: dotted; border-left-width: 100px;"
        );
    }

    #[test]
    fn should_ignore_unsupported_styles() {
        let style: Style = serde_json::from_str(r##"{
            "typography": {
                "fontVariantLigatures": "no-common-ligatures"
            },
            "spacing": { "padding": "10px" }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key, "padding");
        assert_eq!(rules[0].value, "10px");
    }

    #[test]
    fn should_return_a_rules_array_with_css_keys_formatted_in_camelcase() {
        let style1: Style = serde_json::from_str(r##"{
            "spacing": { "padding": "10px", "margin": "12px" }
        }"##).unwrap();
        let options1 = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules1 = get_css_rules(&style1, &options1);
        assert_eq!(rules1.len(), 2);
        assert_eq!(rules1[0].key, "margin");
        assert_eq!(rules1[1].key, "padding");

        let style2: Style = serde_json::from_str(r##"{
            "background": {
                "backgroundImage": {
                    "url": "https://example.com/image.jpg"
                },
                "backgroundPosition": "50% 50%",
                "backgroundRepeat": "no-repeat",
                "backgroundSize": "300px",
                "backgroundAttachment": "fixed"
            },
            "color": {
                "text": "#dddddd",
                "background": "#555555",
                "gradient": "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%)"
            },
            "dimensions": {
                "minHeight": "50vh",
                "minWidth": "25vw"
            },
            "spacing": {
                "padding": { "top": "10px", "bottom": "5px" },
                "margin": { "right": "2em", "left": "1vw" }
            },
            "typography": {
                "fontSize": "2.2rem",
                "fontStyle": "italic",
                "fontWeight": "800",
                "fontFamily": "'Helvetica Neue',sans-serif",
                "lineHeight": "3.3",
                "textColumns": "2",
                "textDecoration": "line-through",
                "letterSpacing": "12px",
                "textTransform": "uppercase"
            },
            "outline": {
                "offset": "2px",
                "width": "4px",
                "style": "dashed",
                "color": "red"
            },
            "shadow": "10px 10px red"
        }"##).unwrap();
        let options2 = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules2 = get_css_rules(&style2, &options2);

        let keys: Vec<&str> = rules2.iter().map(|r| r.key.as_str()).collect();
        assert_eq!(
            keys,
            vec![
                "color",
                "background",
                "backgroundColor",
                "minHeight",
                "minWidth",
                "outlineColor",
                "outlineStyle",
                "outlineOffset",
                "outlineWidth",
                "marginRight",
                "marginLeft",
                "paddingTop",
                "paddingBottom",
                "fontFamily",
                "fontSize",
                "fontStyle",
                "fontWeight",
                "letterSpacing",
                "lineHeight",
                "columnCount",
                "textDecoration",
                "textTransform",
                "boxShadow",
                "backgroundImage",
                "backgroundPosition",
                "backgroundRepeat",
                "backgroundSize",
                "backgroundAttachment"
            ]
        );
    }

    #[test]
    fn should_handle_styles_with_css_vars() {
        let style: Style = serde_json::from_str(r##"{
            "color": {
                "text": "var:preset|color|bomba-picante"
            },
            "spacing": { "padding": "11px" }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector a".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 2);
        assert_eq!(rules[0].key, "color");
        assert_eq!(rules[0].value, "var(--wp--preset--color--bomba-picante)");
        assert_eq!(rules[1].key, "padding");
        assert_eq!(rules[1].value, "11px");
    }

    #[test]
    fn should_output_background_image_value_when_that_value_is_a_string() {
        let style: Style = serde_json::from_str(r##"{
            "background": {
                "backgroundImage": "linear-gradient(to bottom,rgb(255 255 0 / 50%),rgb(0 0 255 / 50%), url('https://example.com/image.jpg')"
            }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key, "backgroundImage");
        assert_eq!(rules[0].value, "linear-gradient(to bottom,rgb(255 255 0 / 50%),rgb(0 0 255 / 50%), url('https://example.com/image.jpg')");
    }

    #[test]
    fn should_comma_separate_background_gradient_and_background_image_into_a_single_background_image_value() {
        let style: Style = serde_json::from_str(r##"{
            "background": {
                "backgroundImage": {
                    "url": "https://example.com/image.jpg"
                },
                "gradient": "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%)"
            }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key, "backgroundImage");
        assert_eq!(rules[0].value, "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%), url( 'https://example.com/image.jpg' )");
    }

    #[test]
    fn should_output_only_gradient_as_background_image_when_no_background_image_is_set() {
        let style: Style = serde_json::from_str(r##"{
            "background": {
                "gradient": "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%)"
            }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key, "backgroundImage");
        assert_eq!(rules[0].value, "linear-gradient(135deg,rgb(255,203,112) 0%,rgb(33,32,33) 42%,rgb(65,88,208) 100%)");
    }

    #[test]
    fn should_resolve_background_gradient_preset_slug_to_css_custom_property() {
        let style: Style = serde_json::from_str(r##"{
            "background": {
                "gradient": "var:preset|gradient|vivid-cyan-blue"
            }
        }"##).unwrap();
        let options = StyleOptions { selector: Some(".some-selector".to_string()) };
        let rules = get_css_rules(&style, &options);
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].key, "backgroundImage");
        assert_eq!(rules[0].value, "var(--wp--preset--gradient--vivid-cyan-blue)");
    }

    #[test]
    fn utils_upper_first() {
        assert_eq!(utils::upper_first("toontown"), "Toontown");
    }

    #[test]
    fn utils_camel_case_join() {
        assert_eq!(utils::camel_case_join(&["toon", "town"]), "toonTown");
    }

    #[test]
    fn utils_get_css_value_from_raw_style() {
        let test_cases = vec![
            ("min(40%, 400px)", "min(40%, 400px)"),
            ("var(--wp--preset--color--yellow-bun)", "var:preset|color|yellow-bun"),
            ("var(--wp--preset--font-size--h-1)", "var:preset|font-size|h1"),
            ("var(--wp--preset--font-size--1-px)", "var:preset|font-size|1px"),
            ("var(--wp--preset--color--orange-11-orange)", "var:preset|color|orange11orange"),
            ("var(--wp--preset--color--heavenly-blue)", "var:preset|color|heavenlyBlue"),
            ("var(--wp--preset--background--dark-secrets-100)", "var:preset|background|dark_Secrets_100"),
        ];

        for (expected, val) in test_cases {
            assert_eq!(get_css_value_from_raw_style(val), expected);
        }

        // Test non-string values using get_css_value_from_raw_style_value
        let int_val = serde_json::Value::Number(1000.into());
        assert_eq!(utils::get_css_value_from_raw_style_value(&int_val), int_val);

        let bool_val = serde_json::Value::Bool(false);
        assert_eq!(utils::get_css_value_from_raw_style_value(&bool_val), bool_val);

        let null_val = serde_json::Value::Null;
        assert_eq!(utils::get_css_value_from_raw_style_value(&null_val), null_val);
    }
}
