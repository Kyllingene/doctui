use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use scraper::Html;

use super::ParseResult;
use crate::item::{AssociatedItemKind, ModuleItemKind};
use crate::link::Link;
use crate::parse::style::Style;
use crate::{err, hierarchy, s, maybe};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant {
    name: Arc<str>,
    definition: Style,
    description: Option<Arc<str>>,
}

pub fn parse(page: &Html) -> ParseResult<Constant> {
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

    let description = maybe!(
        main;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#
    ).map(|d| d
        .text()
        .collect::<String>()
        .into()
    );

    let definition = Style::parse(hierarchy!(
        main;
        r#"pre[class="rust item-decl"]"#,
        "code"
    )?).map_err(|e| err!(InvalidElement, "link", Cow::Owned(e.to_string())))?;

    Ok(
        Constant {
            name,
            definition,
            description,
        }
    )
}
