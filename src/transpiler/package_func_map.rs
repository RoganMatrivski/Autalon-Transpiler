pub struct PackageFuncMap {
    pub package_name: String,
    pub func_name: String,
    pub convert: String,
}

pub fn pkg_fn_map_mapper(val: (&str, &str, &str)) -> PackageFuncMap {
    PackageFuncMap {
        package_name: val.0.to_string(),
        func_name: val.1.to_string(),
        convert: val.2.to_string(),
    }
}
