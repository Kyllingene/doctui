use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use scraper::{ElementRef, Node};

use crate::link::Link;
use crate::Str;

/// A `Vec` of [`StyleModifier`] representing styled text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Style(Vec<StyleModifier>);

impl Style {
    /// Returns an empty style.
    pub fn new() -> Self {
        Style(Vec::new())
    }

    /// Return the plain text without formatting.
    pub fn normal(&self) -> Arc<str> {
        self.iter()
            .map(|s| s.normal().to_string())
            .collect::<String>()
            .into()
    }

    /// Parse an HTML element into style.
    ///
    /// ***NOTE:*** Removes all extraneous details.
    pub fn parse(s: ElementRef<'_>) -> Option<Style> {
        let mut style = Vec::new();

        for child in s.children() {
            if let Some(element) = ElementRef::wrap(child) {
                if let Some(s) = StyleModifier::parse(element) {
                    style.push(s);
                }
            } else if let Node::Text(s) = child.value() {
                style.push(StyleModifier::Normal(s.to_string().into()));
            }
        }

        Some(Style(style))
    }
}

impl Deref for Style {
    type Target = Vec<StyleModifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Style {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StyleModifier {
    Normal(Str),
    Heading(Style, u8),
    Code(Str),
    Pre(Str),
    Bold(Style),
    Italic(Style),
    Link(Style, Link),
    List(Vec<Style>, bool),
    ListItem(Style),
    Other(Style),
}

impl StyleModifier {
    pub fn parse(s: ElementRef<'_>) -> Option<StyleModifier> {
        let name = s.value().name();
        match name {
            "bold" | "b" | "strong" => Some(StyleModifier::Bold(Style::parse(s)?)),
            "italic" | "i" | "emphasis" | "em" => Some(StyleModifier::Italic(Style::parse(s)?)),
            "code" => Some(StyleModifier::Code(s.text().collect::<String>().into())),
            "pre" => Some(StyleModifier::Pre(s.text().collect::<String>().into())),
            "h1" => Some(StyleModifier::Heading(Style::parse(s)?, 1)),
            "h2" => Some(StyleModifier::Heading(Style::parse(s)?, 2)),
            "h3" => Some(StyleModifier::Heading(Style::parse(s)?, 3)),
            "h4" => Some(StyleModifier::Heading(Style::parse(s)?, 4)),
            "h5" => Some(StyleModifier::Heading(Style::parse(s)?, 5)),
            "h6" => Some(StyleModifier::Heading(Style::parse(s)?, 6)),
            "a" => {
                if s.text().next().map_or(false, |s| s == "Run") {
                    return None;
                }

                let link = Link::parse(s.value().attr("href")?)?;
                let text = Style::parse(s)?;
                Some(StyleModifier::Link(text, link))
            }
            "li" => Some(StyleModifier::ListItem(Style::parse(s)?)),
            "ul" | "ol" => {
                let mut style = Vec::new();

                for child in s.children() {
                    if let Some(element) = ElementRef::wrap(child) {
                        let element = Style::parse(element)?;

                        if element.0[0] != StyleModifier::Normal(Cow::Owned("\n".to_string())) {
                            style.push(element);
                        }
                    } else if let Node::Text(s) = child.value() {
                        let t = s.to_string();
                        if &t != "\n" {
                            style.push(Style(vec![StyleModifier::Normal(Cow::Owned(t))]));
                        }
                    }
                }

                Some(StyleModifier::List(style, name == "ol"))
            }
            _ => Some(StyleModifier::Other(Style::parse(s)?)),
        }
    }

    /// Return the plain text without formatting.
    pub fn normal(&self) -> Arc<str> {
        match self {
            StyleModifier::Normal(s) | StyleModifier::Code(s) => s.to_owned().into(),
            StyleModifier::Heading(s, _)
            | StyleModifier::Bold(s)
            | StyleModifier::Italic(s)
            | StyleModifier::ListItem(s)
            | StyleModifier::Other(s)
            | StyleModifier::Link(s, _) => s
                .iter()
                .map(|s| s.normal().to_string())
                .collect::<String>()
                .into(),

            StyleModifier::Pre(s) => {
                let mut out = String::new();

                for line in s.lines() {
                    out.push('\t');
                    out.push_str(line);
                    out.push('\n');
                }

                out.into()
            }

            StyleModifier::List(items, ordered) => {
                let mut out = String::new();
                for (i, item) in items.iter().enumerate() {
                    if *ordered {
                        out.push(' ');
                        out.push_str(&(i + 1).to_string());
                        out.push_str(". ");
                    } else {
                        out.push_str(" - ");
                    }

                    out.push_str(item.normal().deref());
                    out.push('\n');
                }

                out.into()
            }
        }
    }
}
