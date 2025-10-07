#![deny(
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::absolute_paths,
    clippy::allow_attributes_without_reason,
    clippy::arbitrary_source_item_ordering,
    clippy::as_conversions,
    clippy::blanket_clippy_restriction_lints,
    clippy::cast_precision_loss,
    clippy::cognitive_complexity,
    clippy::else_if_without_else,
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::integer_division_remainder_used,
    clippy::iter_over_hash_type,
    clippy::min_ident_chars,
    clippy::missing_docs_in_private_items,
    clippy::mod_module_files,
    clippy::multiple_crate_versions,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    clippy::separated_literal_suffix,
    clippy::shadow_reuse,
    clippy::shadow_unrelated,
    clippy::single_call_fn,
    clippy::single_char_lifetime_names,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::too_many_lines,
    clippy::unwrap_used
)]

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
#[expect(clippy::fn_params_excessive_bools)]
#[expect(clippy::too_many_arguments)]
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
    #[expect(unused_variables)] minify_js: bool,
    preserve_brace_template_syntax: bool,
    preserve_chevron_percent_template_syntax: bool,
    remove_bangs: bool,
    remove_processing_instructions: bool,
) -> String {
    py.detach(move || {
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

#[pymodule(gil_used = false)]
fn _minify_html(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(minify))?;
    Ok(())
}
