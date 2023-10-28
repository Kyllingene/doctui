# `all.html` Structure Documentation

There is one `all.html` for each module in the file tree. It lists, in minimal detail, every item in the module. This applies to crates as well.

## Structure overview
 - Content
    - For every kind of module item:
    - Header with kind name
    - List of items
        - Each item: filepath and Rust path

## HTML Reference

```html
<body>
    <main>
        <div class="width-limiter">
            <section id="main-content" class="content">
                <!-- One for each kind of member item -->
                <!-- repeat -->
                <h3 id="KINDs"><!-- Kind of member item -->s</h3>
                <ul class="all-items">
                    <!-- repeat -->
                    <li><a href="FILEPATH"><!-- Rust item path --></a></li>
                    <!-- /repeat -->
                </ul>
                <!-- /repeat -->
            </section>
        </div>
    </main>
</body>
```

