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

## Options

A description of all options is available in the [minify_html docs](https://docs.rs/minify-html/0.15.0/minify_html/struct.Cfg.html#fields).

The default plugin options are aimed at the best possible minification while maintaining compliance with the specification:

```yaml
plugins:
  - search
  - minify_html:
      do_not_minify_doctype: true
      ensure_spec_compliant_unquoted_attribute_values: true
      keep_closing_tags: false
      keep_html_and_head_opening_tags: false
      keep_spaces_between_attributes: true
      keep_comments: false
      keep_input_type_text_attr: true
      keep_ssi_comments: false
      preserve_brace_template_syntax: false
      preserve_chevron_percent_template_syntax: false
      minify_css: true
      minify_js: true
      remove_bangs: false
      remove_processing_instructions: false
```

## License

[MIT](https://github.com/monosans/mkdocs-minify-html-plugin/blob/main/LICENSE)
