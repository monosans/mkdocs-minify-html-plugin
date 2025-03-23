# mkdocs-minify-html-plugin

[![CI](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml)
[![Downloads](https://static.pepy.tech/badge/mkdocs-minify-html-plugin)](https://pepy.tech/project/mkdocs-minify-html-plugin)

MkDocs plugin for minification using [minify-html](https://github.com/wilsonzlin/minify-html), an extremely fast and smart HTML + JS + CSS minifier.

## Usage

Install:

```bash
pip install -U mkdocs-minify-html-plugin
```

Activate in `mkdocs.yml`:

```yaml
plugins:
  - search
  - minify_html
```

## Options

A description of all options is available in the [minify_html docs](https://docs.rs/minify-html/0.16.4/minify_html/struct.Cfg.html#fields).

The default plugin options are aimed at the best possible minification while maintaining compliance with the specification:

```yaml
plugins:
  - search
  - minify_html:
      allow_noncompliant_unquoted_attribute_values: false
      allow_optimal_entities: false
      allow_removing_spaces_between_attributes: false
      keep_closing_tags: false
      keep_comments: false
      keep_html_and_head_opening_tags: false
      keep_input_type_text_attr: true
      keep_ssi_comments: false
      minify_css: true
      minify_doctype: false
      minify_js: true
      preserve_brace_template_syntax: false
      preserve_chevron_percent_template_syntax: false
      remove_bangs: false
      remove_processing_instructions: false
```

## License

[MIT](https://github.com/monosans/mkdocs-minify-html-plugin/blob/main/LICENSE)
