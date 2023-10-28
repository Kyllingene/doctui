# HTML Structure Documentation

## Disclaimer

These documents are intended to ease initial development. This means there are no guarantees about the quality of the documentation; for an example of what may happen, see `DOC-FORMAT.html`, my initial attempt at documenting the structure.

However, it is my earnest desire to document the structure well and comprehensively. If there is documentation missing, incomplete, poorly formatted, etc., please submit a pull request and it will most likely be accepted.

## Meta

### Overview

This is the documentation detailing the structure of each and every page. A couple pages are encapsulated entirely in one file, namely `index.html` and `all.html`. However, the disparate elements and complicated structure of module items require that multiple files be dedicated to each item.

### File naming

Where a page is entirely defined in one file, that file is named after the page (i.e. `all.html` -> `all.html.md`). Otherwise, they are named as makes sense.

## Item categories

There are two categories of items: module items and associated items. Module items are things attached to a module, such as structs, enums, module-level constants, and functions. Associated items are items attached to a module item, such as methods, associated constants and types, struct fields, or enum variants.

If two items can exist in the same section (especially `repeat` sections), they share the same category. For example, though you may mentally distinguish between methods on a struct and implementors of a trait, the two are rather similar in the actual HTML structure.

