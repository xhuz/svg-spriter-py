use svgcleaner::{cleaner, CleaningOptions, ParseOptions, StyleJoinMode, WriteOptions};

#[allow(dead_code)]
pub fn optimize(raw: &String) -> String {
    let parse_option = ParseOptions {
        parse_comments: true,
        parse_declarations: true,
        parse_unknown_elements: true,
        parse_unknown_attributes: true,
        parse_px_unit: false,
        skip_unresolved_classes: true,
        skip_invalid_attributes: false,
        skip_invalid_css: false,
        skip_paint_fallback: false,
    };
    let clean_option = CleaningOptions {
        remove_unused_defs: true,
        convert_shapes: true,
        remove_title: true,
        remove_desc: true,
        remove_metadata: true,
        remove_dupl_linear_gradients: true,
        remove_dupl_radial_gradients: true,
        remove_dupl_fe_gaussian_blur: true,
        ungroup_groups: true,
        ungroup_defs: true,
        group_by_style: false,
        merge_gradients: true,
        regroup_gradient_stops: true,
        remove_invalid_stops: true,
        remove_invisible_elements: true,
        resolve_use: true,
        remove_version: true,
        remove_unreferenced_ids: true,
        trim_ids: true,
        remove_text_attributes: true,
        remove_unused_coordinates: true,
        remove_default_attributes: true,
        remove_xmlns_xlink_attribute: false,
        remove_needless_attributes: true,
        remove_gradient_attributes: true,
        join_style_attributes: StyleJoinMode::All,
        apply_transform_to_gradients: true,
        apply_transform_to_shapes: true,
        paths_to_relative: true,
        remove_unused_segments: true,
        convert_segments: true,
        apply_transform_to_paths: true,
        coordinates_precision: 6,
        properties_precision: 6,
        paths_coordinates_precision: 8,
        transforms_precision: 8,
    };
    let write_option = WriteOptions::default();

    let mut buf = raw.to_owned().into_bytes();

    let mut doc = cleaner::parse_data(raw, &parse_option).unwrap();

    cleaner::clean_doc(&mut doc, &clean_option, &write_option).unwrap();

    buf.clear();

    cleaner::write_buffer(&doc, &write_option, &mut buf);

    String::from_utf8(buf).unwrap()
}
