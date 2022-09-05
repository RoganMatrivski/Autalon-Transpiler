#[derive(Debug)]
struct FnPkgArgsStruct {
    name: String,
    pkg_name: String,
    args: Vec<String>,
    return_type: String,
}

fn tuples_to_struct(a: (&str, &str, Vec<&str>, &str)) -> FnPkgArgsStruct {
    FnPkgArgsStruct {
        name: a.0.to_string(),
        pkg_name: a.1.to_string(),
        args: a.2.iter().map(|x| x.to_string()).collect(),
        return_type: a.3.to_string(),
    }
}

lazy_static! {
    static ref FN_PKG_ARGS: Vec<FnPkgArgsStruct> = Vec::from(
        [
            ("NavigateToUrl", "builtin", vec!["string"], "WebElement"),
            (
                "GetElementByText",
                "builtin",
                vec!["string", "string"],
                "WebElement"
            ),
            (
                "GetElementByTextExact",
                "builtin",
                vec!["string", "string"],
                "WebElement"
            ),
            (
                "GetElementByString",
                "builtin",
                vec!["string", "ByOption", "string"],
                "WebElement"
            ),
            (
                "GetElementByStringExact",
                "builtin",
                vec!["string", "ByOption", "string"],
                "WebElement"
            ),
            (
                "ClickElementByText",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "ClickElementByTextExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "ClickElementByString",
                "builtin",
                vec!["string", "ByOption", "string"],
                "void"
            ),
            (
                "ClickElementByStringExact",
                "builtin",
                vec!["string", "ByOption", "string"],
                "void"
            ),
            (
                "SendTextToElementByText",
                "builtin",
                vec!["string", "string", "string"],
                "void"
            ),
            (
                "SendTextToElementByTextExact",
                "builtin",
                vec!["string", "string", "string"],
                "void"
            ),
            (
                "SendTextToElementByString",
                "builtin",
                vec!["string", "string", "ByOption", "string"],
                "void"
            ),
            (
                "SendTextToElementByStringExact",
                "builtin",
                vec!["string", "string", "ByOption", "string"],
                "void"
            ),
            (
                "InputDateByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputHtmlByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputNumberTextboxByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputTextboxByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputDropdownUsingTextByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputDropdownUsingIndexByLabelExact",
                "builtin",
                vec!["string", "number"],
                "void"
            ),
            (
                "InputRadioUsingTextByLabelExact",
                "builtin",
                vec!["string", "string"],
                "void"
            ),
            (
                "InputRadioUsingIndexByLabelExact",
                "builtin",
                vec!["string", "number"],
                "void"
            ),
            (
                "GetInputFromLabel",
                "builtin",
                vec!["string"],
                "WebElementExtended"
            ),
            (
                "GetIFrameFromLabel",
                "builtin",
                vec!["string"],
                "WebElementExtended"
            ),
            (
                "GetWindowFromTitle",
                "builtin",
                vec!["string"],
                "WebElementExtended"
            ),
            (
                "GetGroupFromTitle",
                "builtin",
                vec!["string"],
                "WebElementExtended"
            ),
            ("GetAndSwitchToAnyIFrame", "builtin", vec![], "void"),
            ("GetAndSwitchToParentIFrame", "builtin", vec![], "void"),
            ("GetAndSwitchToRootIFrame", "builtin", vec![], "void"),
            ("temp", "builtin", vec!["string", "number"], "void"),
        ]
        .map(tuples_to_struct)
    );
}

pub fn get_pkg_fn_return_type(name: &str, pkg: &str, args: Vec<&str>) -> Result<String, String> {
    // TODO: Remove hardcoded package alias switching
    let pkg = if pkg == "#" { "builtin" } else { pkg };

    let fn_list: Vec<&FnPkgArgsStruct> = FN_PKG_ARGS.iter().filter(|x| x.pkg_name == pkg).collect();

    if fn_list.len() == 0 {
        let cloned_pkg = pkg.clone();
        return Err(format!("Cannot find package \"{}\"", cloned_pkg).to_string());
    };

    let fn_metadata = match fn_list.iter().find(|x| x.name == name) {
        Some(args) => args,
        None => {
            return Err(
                format!("Cannot find function \"{}\" in package \"{}\"", name, pkg).to_string(),
            )
        }
    };

    if fn_metadata.args.len() != args.len() {
        println!("{} | {}", args.len(), fn_metadata.args.len());
        println!("{:?}", args);
        println!("{:?}", fn_metadata.args);
        return Err(format!(
            "Argument supplied for function \"{}\" didn't match. Supplied argument count \"{}\", expected argument count \"{}\"",
            name, args.len(), fn_metadata.args.len()
        ).to_string());
    }

    let args_conv: Vec<String> = args.iter().map(|x| x.to_string()).collect();

    for i in 0..args.len() {
        if args_conv[i] != fn_metadata.args[i] {
            return Err(format!(
                "Argument supplied for function \"{}\" didn't match. Supplied argument type for position {} is \"{}\", expected argument type for position {} is \"{}\"",
                name, i, args_conv[i], i, fn_metadata.args[i]
            ).to_string());
        }
    }

    Ok(fn_metadata.return_type.clone())
}
