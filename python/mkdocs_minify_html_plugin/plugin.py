from __future__ import annotations

import sys

from mkdocs.config import config_options as c
from mkdocs.config.base import Config
from mkdocs.config.defaults import MkDocsConfig
from mkdocs.plugins import BasePlugin
from mkdocs.structure.pages import Page

from mkdocs_minify_html_plugin._minify_html import Minifier

if sys.version_info >= (3, 12):
    from typing import override
else:
    from typing_extensions import override


class MinifyHtmlConfig(Config):
    allow_noncompliant_unquoted_attribute_values = c.Type(bool, default=False)
    allow_optimal_entities = c.Type(bool, default=False)
    allow_removing_spaces_between_attributes = c.Type(bool, default=False)
    keep_closing_tags = c.Type(bool, default=False)
    keep_comments = c.Type(bool, default=False)
    keep_html_and_head_opening_tags = c.Type(bool, default=False)
    keep_input_type_text_attr = c.Type(bool, default=True)
    keep_ssi_comments = c.Type(bool, default=False)
    minify_css = c.Type(bool, default=True)
    minify_doctype = c.Type(bool, default=False)
    minify_js = c.Type(bool, default=True)
    preserve_brace_template_syntax = c.Type(bool, default=False)
    preserve_chevron_percent_template_syntax = c.Type(bool, default=False)
    remove_bangs = c.Type(bool, default=False)
    remove_processing_instructions = c.Type(bool, default=False)


class MinifyHtmlPlugin(BasePlugin[MinifyHtmlConfig]):
    _minifier: Minifier

    @override
    def on_config(self, config: MkDocsConfig) -> MkDocsConfig | None:
        self._minifier = Minifier(**self.config)
        return config

    @override
    def on_post_page(self, output: str, *, page: Page, config: MkDocsConfig) -> str | None:
        return self._minifier.minify(output)

    @override
    def on_post_template(
        self, output_content: str, *, template_name: str, config: MkDocsConfig
    ) -> str | None:
        if template_name.endswith(".html"):
            return self._minifier.minify(output_content)
        return output_content
