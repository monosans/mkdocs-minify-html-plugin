# <img src="https://raw.githubusercontent.com/monosans/mkdocs-minify-html-plugin/main/assets/logo.svg" alt="" width="40" align="top"> mkdocs-minify-html-plugin

[![Downloads](https://static.pepy.tech/badge/mkdocs-minify-html-plugin)](https://pepy.tech/project/mkdocs-minify-html-plugin)

MkDocs plugin that minifies the built HTML pages, including inline CSS and JS, with [minify-html](https://github.com/wilsonzlin/minify-html).

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

The options are the fields of minify-html's [`Cfg`](https://docs.rs/minify-html/latest/minify_html/struct.Cfg.html#fields). The defaults aim for the smallest output that stays spec-compliant:

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
