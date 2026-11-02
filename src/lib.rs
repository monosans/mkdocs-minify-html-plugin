//! The Rust side of `mkdocs-minify-html-plugin`: minifies the HTML, CSS and JS
//! that `MkDocs` writes, through `minify-html`.

use pyo3::prelude::*;

#[pymodule]
mod _minify_html {
    use pyo3::prelude::*;

    /// Minifies HTML with options set once, so that each page only passes
    /// its code.
    #[pyclass(frozen, module = "mkdocs_minify_html_plugin._minify_html")]
    struct Minifier(::minify_html::Cfg);

    #[pymethods]
    impl Minifier {
        #[new]
        #[pyo3(signature = (
            *,
            allow_noncompliant_unquoted_attribute_values,
            allow_optimal_entities,
            allow_removing_spaces_between_attributes,
            keep_closing_tags,
            keep_comments,
            keep_html_and_head_opening_tags,
            keep_input_type_text_attr,
            keep_ssi_comments,
            minify_css,
            minify_doctype,
            minify_js,
            preserve_brace_template_syntax,
            preserve_chevron_percent_template_syntax,
            remove_bangs,
            remove_processing_instructions,
        ))]
        #[expect(clippy::fn_params_excessive_bools, clippy::too_many_arguments)]
        const fn new(
            allow_noncompliant_unquoted_attribute_values: bool,
            allow_optimal_entities: bool,
            allow_removing_spaces_between_attributes: bool,
            keep_closing_tags: bool,
            keep_comments: bool,
            keep_html_and_head_opening_tags: bool,
            keep_input_type_text_attr: bool,
            keep_ssi_comments: bool,
            minify_css: bool,
            minify_doctype: bool,
            minify_js: bool,
            preserve_brace_template_syntax: bool,
            preserve_chevron_percent_template_syntax: bool,
            remove_bangs: bool,
            remove_processing_instructions: bool,
        ) -> Self {
            Self(::minify_html::Cfg {
                allow_noncompliant_unquoted_attribute_values,
                allow_optimal_entities,
                allow_removing_spaces_between_attributes,
                keep_closing_tags,
                keep_comments,
                keep_html_and_head_opening_tags,
                keep_input_type_text_attr,
                keep_ssi_comments,
                minify_css,
                minify_doctype,
                minify_js,
                preserve_brace_template_syntax,
                preserve_chevron_percent_template_syntax,
                remove_bangs,
                remove_processing_instructions,
            })
        }

        #[pyo3(signature = (code, /))]
        fn minify<'py>(
            &self,
            py: Python<'py>,
            code: &str,
        ) -> PyResult<Bound<'py, pyo3::types::PyString>> {
            let minified = py.detach(move || {
                ::minify_html::minify(code.as_bytes(), &self.0)
            });
            pyo3::types::PyString::from_bytes(py, &minified)
        }
    }
}
