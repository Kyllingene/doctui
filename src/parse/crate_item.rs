use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use scraper::Html;

use super::ParseResult;
use crate::item::{AssociatedItemKind, ModuleItemKind};
use crate::link::Link;
use crate::{err, hierarchy, s};

pub type Sections = [Vec<AssociatedItem>; AssociatedItemKind::len()];

pub enum AssociatedItem {
    Declaration(Arc<str>),
    Method {
        name: Arc<str>,
        signature: Arc<str>,
        description: Arc<str>,
        href: Link,
    },
    UnitVariant {
        fields: Vec<Arc<str>>,
    },
    StructVariant {
        fields: HashMap<Arc<str>, Arc<str>>,
    },
}

pub struct CrateItem {
    pub name: Arc<str>,
    pub kind: ModuleItemKind,
    pub description: Arc<str>,

    pub sections: Sections,
    // stable: Option<Arc<str>>, TODO: should this be included?
}

pub fn parse(page: &Html) -> ParseResult<CrateItem> {
    let body = hierarchy!(page; r#"body"#)?;

    let kind = body
        .value()
        .attr("class")
        .ok_or_else(|| err!(InvalidElement, "body", Cow::Owned(body.html()),))?;

    let kind = kind
        .strip_prefix("rustdoc ")
        .ok_or_else(|| err!(InvalidElement, "body class", Cow::Owned(kind.to_string()),))?;

    let kind = ModuleItemKind::parse(kind)
        .ok_or_else(|| err!(InvalidElement, "body class", Cow::Owned(kind.to_string()),))?;

    let main = hierarchy!(
        body;
        "main",
        r#"div[class="width-limiter"]"#,
        r#"section[id="main-content"]"#,
    )?;

    let name = hierarchy!(
        main;
        "h1",
        r##"a[href="#"]"##
    )?
    .text()
    .collect::<String>()
    .into();

    let description = hierarchy!(
        main;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#
    )?
    .text()
    .collect::<String>()
    .into();

    let h2 = s!("h2");
    let headers = body.select(&h2).filter(|h| {
        h.value()
            .attr("class")
            .map_or(false, |c| c.contains("small-section-header"))
    });

    let ss = s!(r#"div"#);
    let mut sections_list = body
        .select(&ss)
        .filter(|s| {
            s.value().attr("id").map_or_else(
                || {
                    s.value()
                        .attr("class")
                        .map_or(false, |c| AssociatedItemKind::parse(c).is_some())
                },
                |i| AssociatedItemKind::parse(i).is_some(),
            )
        })
        .filter_map(|s| {
            let ss = s!("details");
            s.select(&ss)
                .filter(|s| {
                    s.value()
                        .attr("class")
                        .map_or(false, |c| c.contains("toggle"))
                })
                .next()
        });

    let mut sections = Sections::default();

    for header in headers {
        let section_kind = header
            .value()
            .attr("id")
            .map(|i| AssociatedItemKind::parse(i))
            .flatten()
            .ok_or_else(|| err!(InvalidElement, "header", Cow::Owned(header.html())))?;

        let section = sections_list.next().ok_or_else(|| {
            err!(
                InvalidElement,
                "HTML",
                Cow::Borrowed("sections and headers are mismatched")
            )
        })?;

        /* Possible associated item kinds by module item kind:
         *  - Keyword: None
         *  - Struct: 
         *  - Enum: 
         *  - Union: same as Struct
         *  - Primitive: 
         *  - Trait: 
         *  - Macro: 
         *  - Attr: 
         *  - Derive: 
         *  - Fn: 
         *  - Typedef: 
         *  - Const: 
         * */

        // Parse page for constants
        if kind == ModuleItemKind::Constant {
            let const_expr = hierarchy!(
                main;
                r#"pre class="rust item-decl""#,
                "code"
            )?.text().collect::<String>().into();

            sections[AssociatedItemKind::Declaration as usize] = vec![AssociatedItem::Declaration(const_expr)];
        }
    }

    if sections_list.next().is_some() {
        return Err(err!(
            InvalidElement,
            "HTML",
            Cow::Borrowed("sections and headers are mismatched")
        ));
    }

    Ok(CrateItem {
        name,
        kind,
        description,

        sections,
    })
}
