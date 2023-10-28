# Struct Structure Documentation

## HTML Reference

```html
<body>
    <main>
        <section class="content" id="main-content">
            <div class="main-heading">
                <h1>Struct <!-- Name --></h1>
                <span ...>...</span>
            </div>

            <pre class="rust item-decl"><code>
                <!-- Struct definition -->
            </code></pre>

            <!-- Optional -->
            <details class="toggle top-doc" open="">
                <summary ...>...</summary>
                <div class="docblock">
                    <p><!-- Struct documentation --></p>
                </div>
            </details>

            <h2 class="fields small-section-header" id="fields">
                Fields
                <a ...>...</a>
            </h2>
            <!-- One for each field -->
            <!-- repeat -->
            <span class="structfield small-section-header" id="structfield.NAME">
                <a ...>...</a>
                <code>
                    <!-- Field name -->: <!-- Field type -->
                </code>
            </span>

            <!-- Optional -->
            <div class="docblock">
                <p><!-- Field documentation --></p>
            </div>
            <!-- /repeat -->

            <!-- One for each kind of implementation -->
            <!-- repeat -->
            <h2 class="small-section-header" id="KINDs">
                <!-- Kind in human-readable form -->s
                <a ...>...</a>
            </h2>

            <div id="KINDs-list">
                <!-- Varies by kind, see respective `impl-KIND.md` -->
            </div>
            <!-- /repeat -->
        </section>
    </main>
</body>
```

