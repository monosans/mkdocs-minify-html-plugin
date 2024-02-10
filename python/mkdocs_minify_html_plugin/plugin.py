from __future__ import annotations

import sys
from pathlib import Path
from typing import Optional

from mkdocs.config import config_options as c
from mkdocs.config.base import Config
from mkdocs.config.defaults import MkDocsConfig
from mkdocs.plugins import BasePlugin
from mkdocs.structure.pages import Page

from ._minify_html import minify

if sys.version_info >= (3, 12):
    from typing import override
else:
    from typing_extensions import override


class MinifyHtmlConfig(Config):  # type: ignore[no-untyped-call]
    do_not_minify_doctype = c.Type(bool, default=True)
    ensure_spec_compliant_unquoted_attribute_values = c.Type(bool, default=True)
    keep_closing_tags = c.Type(bool, default=False)
    keep_html_and_head_opening_tags = c.Type(bool, default=False)
    keep_spaces_between_attributes = c.Type(bool, default=True)
    keep_comments = c.Type(bool, default=False)
    keep_input_type_text_attr = c.Type(bool, default=True)
    keep_ssi_comments = c.Type(bool, default=False)
    preserve_brace_template_syntax = c.Type(bool, default=False)
    preserve_chevron_percent_template_syntax = c.Type(bool, default=False)
    minify_css = c.Type(bool, default=True)
    minify_js = c.Type(bool, default=True)
    remove_bangs = c.Type(bool, default=False)
    remove_processing_instructions = c.Type(bool, default=False)


class MinifyHtmlPlugin(BasePlugin[MinifyHtmlConfig]):  # type: ignore[no-untyped-call]
    @override
    def on_post_page(
        self, output: str, *, page: Page, config: MkDocsConfig
    ) -> Optional[str]:
        return minify(output, **self.config)

    @override
    def on_post_template(
        self, output_content: str, *, template_name: str, config: MkDocsConfig
    ) -> Optional[str]:
        if Path(template_name).suffix == ".html":
            return minify(output_content, **self.config)
        return output_content
