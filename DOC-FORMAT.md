# Rust HTML Documentation Format

## How this document is formatted

The lists are reflections of the important elements of the HTML structure found in `rustdoc` files. Every level is represented, though not every detail is captured; for example, search bars are omitted since they are irrelevant. ***You must not add a detail without making sure every parent is included.*** These lists should be sufficient to create a parser with little to no external information.

There are a few idioms I am following; please follow them, or submit a PR changing those idioms in their entirety. ***Make sure you are consistent.***

- Each item is as follows:
    - Name: `html tags (and text, if any) defining the element`
    - The name segment should be short, but descriptive
    - The code segment may omit left-and-right angle braces if there it is just a single tag
    - The code segment may omit closing tags; they are considered to be closed when another element of the same or higher precedence is found
    - The code segment *must* include any and all attributes on the tags
    - The code segment may use variables, in the form of ALL CAPS variable names. Here are the ones used thus far:
        - `NAME`: The Rust identifier for the current item, crate, or module
        - `PATH`: The filepath to another file, as in an href
        - `TITLE`: The title of the page or segment
        - `KIND`: The kind of item described, e.g. `mod`, `struct`, `macro`, `keyword`
        - `RUST_PATH`: The Rust path to the current item, crate, or module, e.g. `std::path::Path`
        - `RUST_PARENT`: The Rust path to the current item, minus the item itself.
        - `VERSION`: The semver version string for the current item.
    - You may define new variables as you write; but make sure they are consistent and unambiguous, and add them above before submitting. Alternatively, if there are page-specific variables, you may define them prior to the structure list. If a variable name is self-evident, for example `DESCRIPTION` under a `Description:` item, you may omit a definition.
    - Sometimes, there is a structural component that only contains one relevant detail, but several irrelevant ones (e.g. `<div><h1>Important</h1>Unimportant</div>`). In this case, you may omit the unimportant details like so: `<div><h1>Important</h1>...</div>`. Note that you must, in this case, close all tags for clearness.
    - You may bold any structural elements or details you think especially important.

## Structure

- [O] index.html
    - Verify that the structure is consistent
    - Verify that all useful information has been noted
- [O] all.html
    - Verify that the structure is consistent
    - Verify that all useful information has been noted
- [O] KIND.NAME.html
    - Verify that the structure is consistent
    - Verify that all useful information has been noted

### Crate item (KIND.NAME.html)

- Variables:
    - `SECTION:` The name of a section in a shortcut hyperlink
    - `SECTION_HREF`: The link to a section. Often differs from the name, i.e. `Methods` -> `#implementations`
    - `SECTION_ID`: The identifier (`id` attribute) of a section.
    - `ITEM_HREF`: The link to a section. Often differs from the name, i.e. `AsMut` -> `#{long URL-encoded link}`
    - `ITEM_ID`: The identifier (`id` attribute) of an item. Often differs from the name (see above examples).

- **Body**: `body class="rustdoc KIND"`
    - **Sidebar**: `nav class="sidebar"`
        - Sub-elements: `<div class="sidebar-elems"><section>`
            - Sections:
                - Header: `<h3><a href="#SECTION_HREF">SECTION`
                - Items: `ul class="block"`
                    - Item: `<li><a href="#ITEM_HREF">NAME`
    - **Main:** `<main><div class="width-limiter"><section id="main-content" class="content">`
        - Heading: `div class="main-heading"`
            - Title: `<h1>KIND <a class="KIND" href="#">NAME</a>...</h1>`
            - Stable since: `<span class="out-of-band"><span class="since" title="Stable since Rust version VERSION">VERSION</span>...</span>`
        - Description: `<details class="toggle top-doc" open="">...<div class="docblock>DESCRIPTION</div></details>`
    - Sections:
        - Section: `<h2 id="SECTION" class="SECTION? small-section-header">SECTION...</h2><div id="#SECTION-list"><details class="toggle #SECTION-toggle">`
            - If Fields: Items:
                - Item: `<span id="structfield.NAME" class="structfield small-section-header">...<code>NAME: TYPE</code></span>`
            - If Impl: Items:
                - Item:
                    - Signature: `<summary><section id="ITEM_ID" class="KIND"><h3 class="code-header">SIGNATURE`
                    - Members: `div class="impl-items"`
                        - Member: `details class="toggle method-toggle" open=""`
                            - Signature: `<summary><section id="KIND.NAME" class="KIND #SECTION-impl">...<h4 class="code-header">SIGNATURE</h4></section></summary>`
                            - Description: `<div class='docblock'>DESCRIPTION`

Possible `id`s for items (verified by rg on std):
    - Methods: `implementations`
    - Auto Trait Implementations: `synthetic-implementations`
    - Required methods: `required-methods`
    - Required Associated Types: `required-associated-types`
    - Required Associated Constants: `required-associated-consts`
    - Provided Methods: `provided-methods`
    - Implementors: `implementors`
    - Trait Implementations: `trait-implementations`
    - Blanket Implementations: `blanket-implementations`
    - Methods from OTHERTYPE: `deref-methods-OTHERTYPE`

Possible `KIND`s for items (verified by rg on std):
    - Struct field: `structfield`
    - Enum variant: `variant`
    - Method / associated function: `method`
    - Associated type: `associatedtype`
    - Associated constant: `associatedtype`

### Crate index.html

- **Body**: `body class="rustdoc crate"`
    - **Sidebar**: `nav class="sidebar"`
        - Crate name: `<h2 class="location"><a href="#">Crate NAME`
        - Sub-elements: `div class="sidebar-elems"`
            - List "header": `ul class="block"`
                - Version: `li class="version"`
            - `section`:
                - Shortcuts:
                    - `a href="#KIND"`
    - **Main:** `<main><div class="width-limiter">`
        - **Content:** `section id="main-content" class="content"`
            - Crate name: `<div class="main-heading"><h1>Crate <a class="mod" href="#">NAME`
            - **Description:** `<details class="toggle top-doc" open="">...<div class="docblock">DESCRIPTION</div></details>`
                - Auto-links: `a href="PATH" title="KIND RUST_PATH"`
                - Code: `code`
            - **Crate items:** One of below for each kind
                - Header: `<h2 id="KIND" class="small-section-header"><a href="#KIND">`
                - Items: `ul class="item-table"`
                    - Item: `li`
                        - Name: `<div class="item-name"><a class="KIND" href="PATH" title="NAME in RUST_PARENT">NAME`
                        - Description: `div class="desc docblock-short"`

### Crate all.html

- ***Body:*** `body class="rustdoc mod"`
    - **Sidebar**: `nav class="sidebar"`
        - Crate name: `<h2 class="location"><a href="#">Crate NAME`
        - Sub-elements: `div class="sidebar-elems"`
            - `section`:
                - Shortcuts:
                    - `<li><a href="#KIND">TITLE`
    - **Main:** `<main><div class="width-limiter">`
        - **Content:** `section id="main-content" class="content"`
            - Items: `<h3 id="KINDs">KINDs</h3><ul class="all-items>`
                - Item: `<li><a href="PATH">RUST_PATH`

