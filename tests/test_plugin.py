from __future__ import annotations

import re
from pathlib import Path

import pytest
from mkdocs.commands.build import build
from mkdocs.config import load_config
from mkdocs_minify_html_plugin._minify_html import Minifier  # noqa: PLC2701
from mkdocs_minify_html_plugin.plugin import MinifyHtmlConfig

ROOT = Path(__file__).parent.parent
SITE_URL = "https://example.com/"

PAGE = """\
# Title

<!-- comment -->

Text   with   spaces.

<style>
p  {  color :  red ;  }
</style>

<script>
const  answer  =  42 ;
</script>
"""


def _build(site_root: Path, plugins: str) -> Path:
    docs_dir = site_root / "docs"
    docs_dir.mkdir(parents=True)
    (docs_dir / "index.md").write_text(PAGE, "utf-8")
    config_file = site_root / "mkdocs.yml"
    # Without a site_url, MkDocs leaves the sitemap empty.
    config_file.write_text(f"site_name: Test\nsite_url: {SITE_URL}\nplugins:{plugins}\n", "utf-8")
    build(load_config(str(config_file)))
    return site_root / "site"


def _default_config() -> MinifyHtmlConfig:
    config = MinifyHtmlConfig()
    config.load_dict({})
    assert config.validate() == ([], [])
    return config


def test_pages_and_html_templates_are_minified(tmp_path: Path) -> None:
    plain = _build(tmp_path / "plain", " []")
    minified = _build(tmp_path / "minified", "\n  - minify_html")
    minifier = Minifier(**_default_config())
    # index.html goes through on_post_page, 404.html through on_post_template.
    for name in ("index.html", "404.html"):
        plain_html = (plain / name).read_text("utf-8")
        html = (minified / name).read_text("utf-8")
        assert html == minifier.minify(plain_html)
        assert len(html) < len(plain_html)


def test_non_html_templates_are_not_minified(tmp_path: Path) -> None:
    plain = _build(tmp_path / "plain", " []")
    minified = _build(tmp_path / "minified", "\n  - minify_html")
    sitemap = (minified / "sitemap.xml").read_text("utf-8")
    assert f"<loc>{SITE_URL}</loc>" in sitemap
    assert sitemap == (plain / "sitemap.xml").read_text("utf-8")


@pytest.mark.parametrize("keep_comments", [False, True])
def test_options_come_from_mkdocs_yml(tmp_path: Path, keep_comments: bool) -> None:
    site = _build(
        tmp_path, f"\n  - minify_html:\n      keep_comments: {str(keep_comments).lower()}"
    )
    html = (site / "index.html").read_text("utf-8")
    assert ("<!-- comment -->" in html) is keep_comments


def test_readme_lists_every_option_with_its_default() -> None:
    readme = (ROOT / "README.md").read_text("utf-8")
    documented = {
        name: value == "true"
        for name, value in re.findall(r"^ {6}(\w+): (true|false)$", readme, re.MULTILINE)
    }
    assert documented == dict(_default_config())
