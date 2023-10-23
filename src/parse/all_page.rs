use std::borrow::Cow;
use std::path::Path;

use scraper::Html;

use super::ParseResult;
use crate::item::ModuleItemKind;
use crate::link::Link;
use crate::{err, hierarchy, s};

pub type Sections = [Vec<ModuleItem>; ModuleItemKind::len()];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleItem {
    pub href: Link,
    pub rust_path: String,
}

/// Parses an `all.html` page into its constituent sections,
/// each with its own list of items.
pub fn parse(page: &Html) -> ParseResult<Sections> {
    let content = hierarchy! {
        page;

        "html",
        r#"body[class="rustdoc mod"]"#,
        "main",
        r#"section[id="main-content"], section[class="content"]"#,
    }?;

    let hs = s!("h3");
    let headers = content.select(&hs);

    let ss = s!(r#"ul[class="all-items"]"#);
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
            .map(|p| {
                let a = s!("a");
                let p = p.select(&a).next().unwrap();

                let href = p.value().attr("href").unwrap();
                let rust_path = p.text().collect();

                Ok(ModuleItem {
                    href: Link::file(&Path::new(href)).ok_or_else(|| {
                        err!(InvalidElement, "link", Cow::Owned(href.to_string()))
                    })?,
                    rust_path,
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

    Ok(sections)
}
