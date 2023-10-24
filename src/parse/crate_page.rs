use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;

use scraper::Html;

use super::ParseResult;
use crate::item::ModuleItemKind;
use crate::link::Link;
use crate::parse::style::Style;
use crate::{err, hierarchy, maybe, s};

pub type Sections = [Vec<ModuleItem>; ModuleItemKind::len()];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleItem {
    pub href: Link,
    pub rust_path: Arc<str>,

    pub kind: ModuleItemKind,
    pub description: Option<Style>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Crate {
    pub name: Arc<str>,
    pub version: Arc<str>,

    pub description: Option<Style>,
    pub sections: Sections,
}

pub fn parse(page: &Html) -> ParseResult<Crate> {
    let body = hierarchy!(page; r#"body[class="rustdoc mod crate"]"#)?;

    let sidebar = hierarchy!(body; r#"nav[class="sidebar"]"#)?;
    let version = hierarchy!(sidebar; r#"div[class="sidebar-elems"]"#, r#"ul[class="block"]"#, r#"li[class="version"]"#)?;

    let version = version
        .text()
        .collect::<String>()
        .strip_prefix("Version ")
        .ok_or_else(|| err!(InvalidElement, "version div", Cow::Owned(version.html())))?
        .split_once(|ch: char| ch.is_whitespace())
        .ok_or_else(|| err!(InvalidElement, "version", Cow::Owned(version.html())))?
        .0
        .to_string()
        .into();

    let content =
        hierarchy!(body; "main", r#"div[class="width-limiter"]"#, r#"section[id="main-content"]"#)?;

    let name = hierarchy!(content; r#"div[class="main-heading"]"#, "h1", r#"a[class="mod"]"#)?
        .text()
        .collect::<String>()
        .into();

    let description = maybe!(
        content;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#
    )
    .map(|desc| Style::parse(desc))
    .flatten();

    let hs = s!(r#"h2[class="small-section-header"]"#);
    let headers = content.select(&hs);

    let ss = s!(r#"ul[class="item-table"]"#);
    let mut sections_list = content.select(&ss);

    let mut sections = Sections::default();

    for header in headers {
        let h = header.text().collect::<String>();
        let header = ModuleItemKind::parse(&h)
            .ok_or_else(|| err!(InvalidElement, "header", Cow::Owned(h)))?;

        let section = sections_list.next().ok_or_else(|| {
            err!(
                InvalidElement,
                "HTML",
                Cow::Borrowed("sections and headers are mismatched")
            )
        })?;

        let li = s!("li");
        let paths: Result<Vec<_>, _> = section
            .select(&li)
            .map(|li| {
                let p = hierarchy!(li; r#"div[class="item-name"]"#, "a")?;

                let href = p.value().attr("href").unwrap();
                let title = p
                    .value()
                    .attr("title")
                    .ok_or_else(|| err!(InvalidElement, "link", Cow::Owned(p.html())))?;

                let (kind, rust_path) = title.split_once(" ").ok_or_else(|| {
                    err!(InvalidElement, "link title", Cow::Owned(title.to_string()))
                })?;

                // let kind = p.value()
                //     .attr("class")
                //     .ok_or_else(|| err!(
                //         InvalidElement,
                //         "link",
                //         Cow::Owned(p.html())
                //     ))?;

                let kind = ModuleItemKind::parse(kind).ok_or_else(|| {
                    err!(InvalidElement, "item type", Cow::Owned(kind.to_string()),)
                })?;

                let description = maybe!(li; r#"div[class="desc docblock-short"]"#)
                    .map(|desc| Style::parse(desc))
                    .flatten();

                Ok(ModuleItem {
                    href: Link::file(&Path::new(href)).ok_or_else(|| {
                        err!(InvalidElement, "link", Cow::Owned(href.to_string()))
                    })?,
                    rust_path: rust_path.into(),

                    kind,
                    description,
                })
            })
            .collect();

        sections[header as usize] = paths?;
    }

    if sections_list.next().is_some() {
        return Err(err!(
            InvalidElement,
            "HTML",
            Cow::Borrowed("sections and headers are mismatched")
        ));
    }

    Ok(Crate {
        name,
        version,

        description,
        sections,
    })
}
