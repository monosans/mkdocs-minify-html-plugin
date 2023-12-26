# mkdocs-minify-html-plugin

[![CI](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml)
[![Downloads](https://static.pepy.tech/badge/mkdocs-minify-html-plugin)](https://pepy.tech/project/mkdocs-minify-html-plugin)

MkDocs plugin for minification using [minify-html](https://github.com/wilsonzlin/minify-html), an extremely fast and smart HTML + JS + CSS minifier.

## Usage

Install:

```bash
python -m pip install -U mkdocs-minify-html-plugin
```

Activate in `mkdocs.yml`:

```yaml
plugins:
  - search
  - minify_html
```

## Default options

The default options aim for the best possible minification while maintaining compliance with the specification.

```yaml
plugins:
  - search
  - minify_html:
      # Allow unquoted attribute values in the output to contain characters prohibited by the WHATWG specification (https://html.spec.whatwg.org/multipage/syntax.html#attributes-2). These will still be parsed correctly by almost all browsers.
      allow_noncompliant_unquoted_attribute_values: false
      # Allow removing_spaces between attributes when possible, which may not be spec compliant. These will still be parsed correctly by almost all browsers.
      allow_removing_spaces_between_attributes: false
      # Do not omit closing tags when possible.
      keep_closing_tags: false
      # Keep all comments.
      keep_comments: false
      # Do not omit `<html>` and `<head>` opening tags when they don't have attributes.
      keep_html_and_head_opening_tags: false
      # Keep `type=text` attribute name and value on `<input>` elements.
      keep_input_type_text_attr: true
      # Keep SSI comments.
      keep_ssi_comments: false
      # Minify CSS in `<style>` tags and `style` attributes using lightningcss (https://github.com/parcel-bundler/lightningcss).
      minify_css: true
      # Minify DOCTYPEs. Minified DOCTYPEs may not be spec compliant, but will still be parsed correctly by almost all browsers.
      minify_doctype: false
      # Minify JavaScript in `<script>` tags using minify-js (https://github.com/wilsonzlin/minify-js).
      minify_js: true
      # When `{{`, `{#`, or `{%` are seen in content, all source code until the subsequent matching closing `}}`, `#}`, or `%}` respectively gets piped through untouched.
      preserve_brace_template_syntax: false
      # When `<%` is seen in content, all source code until the subsequent matching closing `%>` gets piped through untouched.
      preserve_chevron_percent_template_syntax: false
      # Remove all bangs.
      remove_bangs: false
      # Remove all processing instructions.
      remove_processing_instructions: false
```

## License

[MIT](https://github.com/monosans/mkdocs-minify-html-plugin/blob/main/LICENSE)
