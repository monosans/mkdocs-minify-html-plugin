use pyo3::prelude::*;

#[pyfunction]
#[pyo3(signature = (
    code,
    /,
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
#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::too_many_arguments)]
fn minify(
    py: Python<'_>,
    code: &str,
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
    #[allow(unused_variables)] minify_js: bool,
    preserve_brace_template_syntax: bool,
    preserve_chevron_percent_template_syntax: bool,
    remove_bangs: bool,
    remove_processing_instructions: bool,
) -> String {
    py.allow_threads(move || {
        let cfg = ::minify_html::Cfg {
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
            minify_js: false,
            preserve_brace_template_syntax,
            preserve_chevron_percent_template_syntax,
            remove_bangs,
            remove_processing_instructions,
        };
        let minified = ::minify_html::minify(code.as_bytes(), &cfg);
        String::from_utf8(minified).unwrap()
    })
}

#[pymodule]
fn _minify_html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(minify))?;
    Ok(())
}
