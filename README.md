# mkdocs-minify-html-plugin

[![CI](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/monosans/mkdocs-minify-html-plugin/actions/workflows/ci.yml)
[![Downloads](https://static.pepy.tech/badge/mkdocs-minify-html-plugin)](https://pepy.tech/project/mkdocs-minify-html-plugin)

MkDocs plugin for minification using [minify-html](https://github.com/wilsonzlin/minify-html), an extremely fast HTML + JS + CSS minifier.

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

- [do_not_minify_doctype](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.do_not_minify_doctype) = True
- [ensure_spec_compliant_unquoted_attribute_values](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.ensure_spec_compliant_unquoted_attribute_values) = True
- [keep_closing_tags](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.keep_closing_tags) = False
- [keep_html_and_head_opening_tags](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.keep_html_and_head_opening_tags) = False
- [keep_spaces_between_attributes](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.keep_spaces_between_attributes) = True
- [keep_comments](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.keep_comments) = False
- [minify_css](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.minify_css) = True
- [minify_js](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.minify_js) = True
- [remove_bangs](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.remove_bangs) = False
- [remove_processing_instructions](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#structfield.remove_processing_instructions) = False

Example:

```yaml
plugins:
  - search
  - minify_html:
      do_not_minify_doctype: false
      ensure_spec_compliant_unquoted_attribute_values: false
      keep_spaces_between_attributes: false
```

## License

[MIT](https://github.com/monosans/mkdocs-minify-html-plugin/blob/main/LICENSE)
