use venial::Attribute;

// Check attribute name
pub(crate) fn check_attr_name(attr: &Attribute, name: &str) -> bool {
    if let Some(attr_name) = attr.get_single_path_segment() {
        if attr_name == name {
            return true;
        }
    };
    false
}
