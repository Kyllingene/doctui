use std::borrow::Cow;
use std::sync::Arc;

use scraper::Html;

use super::impls::{self, Impl};
use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: Arc<str>,
    pub definition: Style,
    pub description: Option<Style>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Member {
    pub name: Arc<str>,
    pub kind: AssociatedItemKind,

    pub signature: Style,
    pub description: Option<Style>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    pub name: Arc<str>,
    pub description: Option<Style>,
    pub kind: ModuleItemKind,

    pub fields: Vec<Field>,
    pub impls: Vec<Impl>,
}

pub fn parse(page: &Html) -> ParseResult<Struct> {
    let content = hierarchy!(
        page;
        "body",
        "main",
        r#"div[class="width-limiter"]"#,
        r#"section[id="main-content"]"#
    )?;

    let head = hierarchy!(
        content;
        r#"div[class="main-heading"]"#,
        "h1",
        r##"a[href="#"]"##,
    )?;

    let name = head.text().collect::<String>().into();

    let kind = head
        .value()
        .attr("class")
        .and_then(|class| ModuleItemKind::parse(class))
        .unwrap_or(ModuleItemKind::Struct);

    let description = maybe!(
        content;
        r#"details[class="toggle top-doc"]"#,
        r#"div[class="docblock"]"#
    )
    .map(|desc| Style::parse(desc))
    .flatten();

    let hs = s!("h2");
    let headers = content.select(&hs);

    let fs = s!(r#"span[class="structfield small-section-header"]"#);
    let fields_list = content.select(&fs);

    let ss = s!("div");
    let mut sections_list = content.select(&ss).filter(|s| {
        s.value()
            .attr("id")
            .map_or(false, |id| AssociatedItemKind::parse(id).is_some())
    });

    // TODO: redo this whole thing to capture field documentation (ugh)
    let mut fields = Vec::new();
    for field_el in fields_list {
        let name = field_el
            .value()
            .attr("id")
            .ok_or_else(|| err!(InvalidElement, "struct field", Cow::Owned(field_el.html())))?
            .strip_prefix("structfield.")
            .ok_or_else(|| err!(InvalidElement, "struct field", Cow::Owned(field_el.html())))?;

        let mut definition = Style::parse(hierarchy!(
            field_el;
            "code"
        )?)
        .ok_or_else(|| err!(InvalidElement, "struct field", Cow::Owned(field_el.html())))?;

        {
            let definition: &mut Vec<StyleModifier> = &mut definition;
            let first = definition
                .get_mut(0)
                .ok_or_else(|| err!(InvalidElement, "struct field", Cow::Owned(field_el.html())))?;

            let StyleModifier::Normal(first) = first else {
                return Err(err!(
                    InvalidElement,
                    "struct field",
                    Cow::Owned(field_el.html())
                ));
            };

            *first = Cow::Owned(
                first
                    .strip_prefix(&format!("{name}: "))
                    .ok_or_else(|| {
                        err!(InvalidElement, "struct field", Cow::Owned(field_el.html()))
                    })?
                    .to_string(),
            );
        }

        let name = name.into();

        fields.push(Field {
            name,
            definition,
            description: None,
        });
    }

    let impls = impls::parse_all(content)?;

    Ok(Struct {
        name,
        description,
        kind,

        fields,
        impls,
    })
}
