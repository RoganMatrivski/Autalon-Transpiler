use std::str::FromStr;

use crate::builtin_pkg_definition;

pub fn get_pkg_fn_return_type(name: &str, pkg: &str, args: Vec<&str>) -> Result<String, String> {
    // TODO: Remove hardcoded package alias switching
    let pkg = if pkg == "#" { "builtin" } else { pkg };

    let fn_metadata = match pkg {
        "builtin" => {
            let fn_enum = match builtin_pkg_definition::BuiltinPkgFunctions::from_str(name) {
                Ok(res) => res,
                Err(err) => panic!("{}", err.to_string()),
            };

            builtin_pkg_definition::get_fn_metadata(&fn_enum)
        }
        str => unimplemented!("Package {} is currently unimplemented.", str),
    };

    if fn_metadata.args.len() != args.len() {
        return Err(format!(
            "Argument supplied for function \"{}\" didn't match. Supplied argument count \"{}\", expected argument count \"{}\"",
            name, args.len(), fn_metadata.args.len()
        ).to_string());
    }

    let args_conv: Vec<String> = args.iter().map(|x| x.to_string()).collect();

    for i in 0..args.len() {
        if args_conv[i] != fn_metadata.args[i].arg_type {
            return Err(format!(
                "Argument supplied for function \"{}\" didn't match. Supplied argument type for position {} is \"{}\", expected argument type for position {} is \"{}\"",
                name, i, args_conv[i], i, fn_metadata.args[i].arg_type
            ).to_string());
        }
    }

    Ok(fn_metadata.return_type.clone())
}
