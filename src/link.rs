#![allow(unused)]

use std::borrow::Cow;
use std::path::{Component, Path};
use std::sync::Arc;

use crate::item::ModuleItemKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Link {
    File {
        parents: Vec<Cow<'static, str>>,
        kind: Option<ModuleItemKind>,
        name: Arc<str>,
    },
    Anchor(Arc<str>),
    Url(Arc<str>),
}

/// A struct representing a link to or within
/// a doc page, or to a web page. Can easily be
/// converted to/from both rustdoc HTML links
/// and doctui links.
impl Link {
    /// Parse a link that could be path-style, anchor-style, or url-style.
    pub fn parse(link: &str) -> Option<Self> {
        if let Some(anchor) = Self::anchor(link) {
            Some(anchor)
        } else if let Some(url) = Self::url(link) {
            Some(url)
        } else {
            Self::file(&Path::new(link))
        }
    }

    /// Parse a path-style link (e.g. `path/struct.Path.html`).
    pub fn file(file: &Path) -> Option<Self> {
        let mut parents = Vec::new();

        for component in file.components() {
            match component {
                Component::RootDir => parents.push(Cow::Borrowed("/")),
                Component::CurDir => {}
                Component::ParentDir => parents.push(Cow::Borrowed("..")),
                Component::Normal(s) => parents.push(Cow::Owned(s.to_string_lossy().to_string())),
                Component::Prefix(s) => {
                    parents.push(Cow::Owned(s.as_os_str().to_string_lossy().to_string()))
                }
            }
        }

        // Ensures that it isn't `/` or `..` for free
        let Cow::Owned(filename) = parents.pop()? else {
            return None;
        };
        let mut parts = filename.split('.');

        let kind = parts.next()?;
        let name = parts.next()?;

        let kind = ModuleItemKind::parse(kind);

        Some(Self::File {
            parents,
            kind,
            name: name.into(),
        })
    }

    /// Parse an anchor-style link (e.g. `#implementations`).
    pub fn anchor(anchor: &str) -> Option<Self> {
        Some(Self::Anchor(anchor.strip_prefix('#')?.into()))
    }

    /// Parse a URL-style link (e.g. `https://rust-lang.org`).
    pub fn url(url: &str) -> Option<Self> {
        Some(Self::Url(url.strip_prefix("https://")?.into()))
    }

    /// Convert a link back into rustdoc (HTML) form.
    pub fn to_rustdoc(&self) -> String {
        match self {
            Self::File {
                parents,
                kind,
                name,
            } => {
                let mut out = parents.join(std::path::MAIN_SEPARATOR_STR);
                out.push(std::path::MAIN_SEPARATOR);

                if let Some(kind) = kind {
                    out.push_str(kind.to_keyword());
                    out.push('.');
                }

                out.push_str(name.as_ref());
                out.push_str(".html");

                out
            }
            Self::Anchor(anchor) => format!("#{anchor}"),
            Self::Url(url) => format!("https://{url}"),
        }
    }
}
