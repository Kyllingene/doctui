# Implementation Block Structure Documentation

## HTML Reference

```html
<h2 class="small-section-header" id="KINDs">
    <!-- Associated item kind in human-readable form -->
    <a ...>§</a>
</h2>
<!-- Note that this ID may be re-used for different kinds; rely on the header ID for actual identification -->
<div id="KINDs-list">
    <!-- One for each item -->
    <!-- repeat -->
    <!-- If the item has documentation, add the following wrapper:
        <details class="toggle ?method-toggle?" open="">
            ...
        </details>
    -->
    <summary>
        <section class="KIND" id="KIND.NAME">
            <a ...>...</a>
            <h4 class="code-header">
                <!--
                    The definition of the item as in Rust.

                    For items such as associated constants, this includes the value. For items such as methods, however, it omits the actual implementation, leaving just the signature.

                    Should be parsed as a styled string, since it can (and usually does) include links to other items.
                -->
            </h4>
        </section>
    </summary>
    <!-- Optional -->
    <div class="docblock">
        <p>
            <!-- Item documentation -->
        </p>
    </div>
    <!-- /repeat -->
</div>
```

