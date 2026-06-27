use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<Background>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<Border>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<Spacing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<Typography>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Elements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline: Option<Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Background {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_image: Option<BackgroundImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_position: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_repeat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_attachment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BackgroundImage {
    Value(String),
    Object {
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        source: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Border {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<BorderRadius>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<BorderIndividualStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<BorderIndividualStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<BorderIndividualStyles>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<BorderIndividualStyles>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum BorderRadius {
    Value(String),
    Object {
        #[serde(rename = "topLeft", skip_serializing_if = "Option::is_none")]
        top_left: Option<String>,
        #[serde(rename = "topRight", skip_serializing_if = "Option::is_none")]
        top_right: Option<String>,
        #[serde(rename = "bottomLeft", skip_serializing_if = "Option::is_none")]
        bottom_left: Option<String>,
        #[serde(rename = "bottomRight", skip_serializing_if = "Option::is_none")]
        bottom_right: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BorderIndividualStyles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Dimensions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Spacing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<ValueOrBox>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<ValueOrBox>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValueOrBox {
    Value(String),
    Box(BoxModel),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct BoxModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Typography {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_height: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_columns: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_decoration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_indent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writing_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Elements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<LinkElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinkElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<LinkColor>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct LinkColor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Outline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct StyleOptions {
    pub selector: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedCSSRule {
    pub selector: Option<String>,
    pub key: String,
    pub value: String,
}
