use pyo3::prelude::*;

#[allow(clippy::fn_params_excessive_bools)]
#[allow(clippy::too_many_arguments)]
#[pyfunction]
#[pyo3(signature = (
    code,
    /,
    *,
    do_not_minify_doctype,
    ensure_spec_compliant_unquoted_attribute_values,
    keep_closing_tags,
    keep_html_and_head_opening_tags,
    keep_spaces_between_attributes,
    keep_comments,
    keep_input_type_text_attr,
    keep_ssi_comments,
    preserve_brace_template_syntax,
    preserve_chevron_percent_template_syntax,
    minify_css,
    minify_js,
    remove_bangs,
    remove_processing_instructions,
))]
fn minify(
    py: Python,
    code: &str,
    do_not_minify_doctype: bool,
    ensure_spec_compliant_unquoted_attribute_values: bool,
    keep_closing_tags: bool,
    keep_html_and_head_opening_tags: bool,
    keep_spaces_between_attributes: bool,
    keep_comments: bool,
    keep_input_type_text_attr: bool,
    keep_ssi_comments: bool,
    preserve_brace_template_syntax: bool,
    preserve_chevron_percent_template_syntax: bool,
    minify_css: bool,
    minify_js: bool,
    remove_bangs: bool,
    remove_processing_instructions: bool,
) -> String {
    py.allow_threads(move || {
        let cfg = ::minify_html::Cfg {
            do_not_minify_doctype,
            ensure_spec_compliant_unquoted_attribute_values,
            keep_closing_tags,
            keep_html_and_head_opening_tags,
            keep_spaces_between_attributes,
            keep_comments,
            keep_input_type_text_attr,
            keep_ssi_comments,
            preserve_brace_template_syntax,
            preserve_chevron_percent_template_syntax,
            minify_css,
            minify_js,
            remove_bangs,
            remove_processing_instructions,
        };
        let minified = ::minify_html::minify(code.as_bytes(), &cfg);
        String::from_utf8(minified).unwrap()
    })
}

#[pymodule]
fn _minify_html(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(minify))?;
    Ok(())
}
