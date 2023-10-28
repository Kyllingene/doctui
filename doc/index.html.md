# `index.html` Structure Documentation

There is one `index.html` for every module, including crates. It provides crate-level documentation, as well as an overview of each item, with documentation.

## Structure overview

- Content:
    - Crate name
    - Optional crate documentation
    - For each kind of module item:
    - Kind name
    - List of items
        - Item:
        - Kind
        - Filepath
        - Name
        - Parent
        - Optional documentation

## HTML Reference

```html
<body>
    <main>
        <div class="width-limiter">
            <section id="main-content" class="content">
                <div class="main-heading">
                    <h1>Crate <a class="mod" href="#"><!-- Crate name --></a></h1>
                </div>

                <!-- Optional -->
                <details class="toggle top-doc" open="">
                    <!-- Junk such as `source` link and doc collapser --> ...
                    <div class="docblock">
                        <!-- Crate documentation -->
                    </div>
                </details>

                <!-- One of the following for each kind -->
                <!-- repeat -->
                <h2 id="KIND" class="small-section-header">
                    <a href="#KIND"><!-- Kind in human-readable form --></a>
                </h2>
                <ul class="item-table">
                    <li>
                        <div class="item-name">
                            <a class="KIND" href="PATH" title="NAME in RUST_PARENT">
                                <!-- Name of item in human-readable form -->
                            </a>
                        </div>
                        <!-- Optional -->
                        <div class="desc docblock-short">
                            <!-- Item documentation -->
                        </div>
                    </li>
                </ul>
                <!-- /repeat -->
            </section>
        </div>
    </main>
</body>
```

