# Changelog

[Semantic Versioning](https://semver.org/)

## [0.3.3] - 2025-07-29

- Remove loongarch64-unknown-linux-gnu target as it is not supported by PyPI.

## [0.3.2] - 2025-07-29

- Add CPython 3.14 and free-threaded CPython 3.14 support.
- Add riscv64gc-unknown-linux-gnu and loongarch64-unknown-linux-gnu targets support.
- Don't build x86_64-unknown-linux-gnu and aarch64-unknown-linux-gnu manylinux_2_34 wheels as they are in alpha.

## [0.3.1] - 2025-03-23

- Don't minify javascript until <https://github.com/wilsonzlin/minify-html/issues/225> is fixed.

## [0.3.0] - 2025-03-23

- Bump `minify_html` from v0.15.0 to v0.16.4. Some config options were renamed, see [changelog](https://github.com/wilsonzlin/minify-html/blob/v0.16.4/CHANGELOG.md).

## [0.2.5] - 2025-02-25

- Add support for PyPy 3.11 and free-threaded CPython 3.13.

## [0.2.4] - 2025-02-01

- Require Python >= 3.9.
- Build for more platforms and manylinux versions to improve performance on modern systems.

## [0.2.3] - 2024-08-20

- Add Python 3.13 support.
- Rebuild with updated compiler and dependencies.

## [0.2.2] - 2024-05-18

- Rebuild with updated compiler and dependencies.

## [0.2.1] - 2024-02-21

- Unpin dependencies.
- Don't require typing-extensions on Python >= 3.12.

## [0.2.0] - 2023-12-26

- Bundle `minify-html` with the plugin to avoid depending on the PyPI package. This version uses `minify-html` v0.15.0.
- Lower the minimum required version of Python from 3.8 to 3.7.

## [0.1.0] - 2023-07-08

- Initial release.
