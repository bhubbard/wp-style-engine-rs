# wp-style-engine-rs

Rust port of [`@wordpress/style-engine`](https://github.com/WordPress/gutenberg/tree/trunk/packages/style-engine) from the Gutenberg project.

Compiles Gutenberg block `style` attribute objects into inline CSS strings or scoped stylesheets. Handles color, spacing, typography, border, outline, background, and shadow — including `var:preset|*` custom property resolution.

## Installation

```toml
[dependencies]
wp-style-engine-rs = "0.1"
```

## Usage

```rust
use wp_style_engine_rs::{compile_css, Style, StyleOptions};
use serde_json::json;

// Build a style object
let style: Style = serde_json::from_value(json!({
    "color": { "text": "#333333", "background": "#ffffff" },
    "spacing": { "padding": { "top": "1rem", "bottom": "1rem" } },
    "typography": { "fontSize": "1.125rem", "fontWeight": "600" }
})).unwrap();

// Inline styles (no selector)
let inline = compile_css(&style, &StyleOptions::default());
// => "color: #333333; background-color: #ffffff; padding-top: 1rem; ..."

// Scoped stylesheet (with selector)
let opts = StyleOptions { selector: Some(".wp-block-heading".into()), ..Default::default() };
let sheet = compile_css(&style, &opts);
// => ".wp-block-heading { color: #333333; ... }"
```

## Features

- Full `var:preset|color|*` → `var(--wp--preset--color--*)` resolution
- Supports nested selectors (`:hover`, `::placeholder`)
- Generates scoped rules or inline `style` attribute strings
- 18 unit tests matching Gutenberg's own test suite

## Related Crates

| Crate | Purpose |
|---|---|
| [`wp-block-parser-rs`](https://crates.io/crates/wp-block-parser-rs) | Parse Gutenberg block markup |
| [`wp-token-list-rs`](https://crates.io/crates/wp-token-list-rs) | Manage block CSS class tokens |
| [`wp-escape-html-rs`](https://crates.io/crates/wp-escape-html-rs) | Sanitize block attribute values |

## License

GPL-2.0-or-later — consistent with the [Gutenberg project](https://github.com/WordPress/gutenberg).
