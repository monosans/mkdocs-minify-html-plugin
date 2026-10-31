<!--
Pull requests are squash-merged, so write the title in the Conventional Commits
style: `fix: ...`, `feat: ...`, `docs: ...`, `refactor: ...`, `test: ...`, `chore: ...`.
-->

## Summary

<!-- What does this change, and why? -->

## Related issues

<!-- `Closes #123` closes the issue on merge. Remove this section if there are none. -->

## Testing

<!-- How did you check that this works? -->

## Checklist

- [ ] `uvx prek run --all-files` passes.
- [ ] `cargo +nightly fmt --check` and `cargo +beta clippy --all-targets --` pass, if Rust code changed.
- [ ] `uv run pytest` passes, and new behavior has tests.
- [ ] `README.md` and `python/mkdocs_minify_html_plugin/_minify_html.pyi` match any change to the options.
