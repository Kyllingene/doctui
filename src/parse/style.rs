use std::borrow::Cow;
use std::ops::Deref;
use std::sync::Arc;

use scraper::{ElementRef, Node};

use crate::Str;
use crate::link::Link;

/// A `Vec` of [`StyleModifier`] representing styled text.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Style(Vec<StyleModifier>);

impl Style {
    /// Return the plain text without formatting.
    pub fn normal(&self) -> Arc<str> {
        self.iter().map(|s| s.normal().to_string()).collect::<String>().into()
    }

    /// Parse an HTML element into style.
    /// 
    /// ***NOTE:*** Removes all extraneous details.
    pub fn parse(s: ElementRef<'_>) -> Result<Style, Arc<str>> {
        let mut style = Vec::new();

        for child in s.children() {
            if let Some(element) = ElementRef::wrap(child) {
                style.push(StyleModifier::parse(element)?);
            } else if let Node::Text(s) = child.value() {
                style.push(StyleModifier::Normal(s.to_string().into()));
            }
        }

        Ok(Style(style))
    }
}

impl Deref for Style {
    type Target = Vec<StyleModifier>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StyleModifier {
    Normal(Str),
    Heading(Style, u8),
    Code(Str),
    Bold(Style),
    Italic(Style),
    Link(Style, Link),
}

impl StyleModifier {
    pub fn parse(s: ElementRef<'_>) -> Result<StyleModifier, Arc<str>> {
        match s.value().name() {
            "bold" | "b" | "strong" => Ok(StyleModifier::Bold(Style::parse(s)?)),
            "italic" | "i" | "emphasis" | "em" => Ok(StyleModifier::Italic(Style::parse(s)?)),
            "code" | "pre" => Ok(StyleModifier::Code(s.text().collect::<String>().into())),
            "h1" => Ok(StyleModifier::Heading(Style::parse(s)?, 1)),
            "h2" => Ok(StyleModifier::Heading(Style::parse(s)?, 2)),
            "h3" => Ok(StyleModifier::Heading(Style::parse(s)?, 3)),
            "h4" => Ok(StyleModifier::Heading(Style::parse(s)?, 4)),
            "h5" => Ok(StyleModifier::Heading(Style::parse(s)?, 5)),
            "h6" => Ok(StyleModifier::Heading(Style::parse(s)?, 6)),
            "a" => {
                let e = || Into::<Arc<str>>::into(s.html());
                let link = Link::parse(s.value().attr("href").ok_or_else(e)?).ok_or_else(e)?;
                let text = Style::parse(s)?;

                Ok(StyleModifier::Link(text, link))
            }
            _ => Ok(StyleModifier::Normal(s.text().collect::<String>().into())),
        }
    }

    /// Format the style with ANSI escape codes.
    pub fn to_ansi(&self) -> Arc<str> {
        todo!()
    }

    /// Return the plain text without formatting.
    pub fn normal(&self) -> Arc<str> {
        match self {
            StyleModifier::Normal(s)
            | StyleModifier::Code(s) => s.to_owned().into(),
            StyleModifier::Heading(s, _)
            | StyleModifier::Bold(s)
            | StyleModifier::Italic(s)
            | StyleModifier::Link(s, _) => s.iter().map(|s| s.normal().to_string()).collect::<String>().into(),
        }
    }
}

