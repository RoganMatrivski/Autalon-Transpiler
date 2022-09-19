use serde::Serialize;

use strum_macros::{Display, EnumIter, EnumString};

use crate::ostr;

enum ArgType {
    String,
    Number,
    Bool,
    ByOption,
}

#[derive(PartialEq, EnumString, EnumIter, Display)]
pub enum BuiltinPkgFunctions {
    NavigateToUrl,
    GetElementByString,
    ClickElementByString,
    SendTextToElementByString,
    GetInputFromLabel,
    GetIFrameFromLabel,
    GetWindowFromLabel,
    GetGroupFromLabel,
    InputDateByLabelExact,
    InputHtmlByLabelExact,
    InputNumberTextboxByLabelExact,
    InputTextboxByLabelExact,
    InputDropdownUsingTextByLabelExact,
    InputDropdownUsingIndexByLabelExact,
    InputRadioUsingTextByLabelExact,
    InputRadioUsingIndexByLabelExact,
    GetAndSwitchToAnyIFrame,
    GetAndSwitchToParentIFrame,
    GetAndSwitchToRootIFrame,

    SetWindowDimension,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionArgsMetadata {
    pub display_name: String,
    pub description: String,
    pub arg_type: String,
    pub default_value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub args: Vec<FunctionArgsMetadata>,
    pub return_type: String,
}

pub fn get_fn_metadata(builtin_fn: &BuiltinPkgFunctions) -> FunctionMetadata {
    match builtin_fn {
        BuiltinPkgFunctions::NavigateToUrl => FunctionMetadata {
            name: ostr!("NavigateToUrl"),
            display_name: ostr!("Navigate To Url"),
            description: ostr!("Navigate current page to Url"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("URL"),
                description: ostr!("URL to navigate to"),
                arg_type: ostr!("string"),
                default_value: ostr!("https://www.google.com"),
            }],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::GetElementByString => FunctionMetadata {
            name: ostr!("GetElementByString"),
            display_name: ostr!("Get Element By String"),
            description: ostr!("Get element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("By Option"),
                    description: ostr!("The attribute to match with the string"),
                    arg_type: ostr!("ByOption"),
                    default_value: ostr!("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Exact Match"),
                    description: ostr!("Match the whole string"),
                    arg_type: ostr!("bool"),
                    default_value: ostr!("ByOption.Text"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::ClickElementByString => FunctionMetadata {
            name: ostr!("ClickElementByString"),
            display_name: ostr!("Click Element By String"),
            description: ostr!("Click element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("By Option"),
                    description: ostr!("The attribute to match with the string"),
                    arg_type: ostr!("ByOption"),
                    default_value: ostr!("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Exact Match"),
                    description: ostr!("Match the whole string"),
                    arg_type: ostr!("bool"),
                    default_value: ostr!("ByOption.Text"),
                },
            ],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::SendTextToElementByString => FunctionMetadata {
            name: ostr!("SendTextToElementByString"),
            display_name: ostr!("Send text to Element By String"),
            description: ostr!("Send text to element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("String to input"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("By Option"),
                    description: ostr!("The attribute to match with the string"),
                    arg_type: ostr!("ByOption"),
                    default_value: ostr!("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Exact Match"),
                    description: ostr!("Match the whole string"),
                    arg_type: ostr!("bool"),
                    default_value: ostr!("ByOption.Text"),
                },
            ],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::GetInputFromLabel => FunctionMetadata {
            name: ostr!("GetInputFromLabel"),
            display_name: ostr!("Get Input From Label"),
            description: ostr!("Get input element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetIFrameFromLabel => FunctionMetadata {
            name: ostr!("GetIFrameFromLabel"),
            display_name: ostr!("Get IFrame From Label"),
            description: ostr!("Get IFrame element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetWindowFromLabel => FunctionMetadata {
            name: ostr!("GetWindowFromLabel"),
            display_name: ostr!("Get Window From Label"),
            description: ostr!("Get window element from title text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetGroupFromLabel => FunctionMetadata {
            name: ostr!("GetGroupFromLabel"),
            display_name: ostr!("Get Group From Label"),
            description: ostr!("Get group element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::InputDateByLabelExact => FunctionMetadata {
            name: ostr!("InputDateByLabelExact"),
            display_name: ostr!("Input Date By Label Exact"),
            description: ostr!("Input date on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputHtmlByLabelExact => FunctionMetadata {
            name: ostr!("InputHtmlByLabelExact"),
            display_name: ostr!("Input Html By Label Exact"),
            description: ostr!("Input html on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputNumberTextboxByLabelExact => FunctionMetadata {
            name: ostr!("InputNumberTextboxByLabelExact"),
            display_name: ostr!("Input Number Textbox By Label Exact"),
            description: ostr!("Input number textbox on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputTextboxByLabelExact => FunctionMetadata {
            name: ostr!("InputTextboxByLabelExact"),
            display_name: ostr!("Input Textbox By Label Exact"),
            description: ostr!("Input textbox on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputDropdownUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("InputDropdownUsingTextByLabelExact"),
            display_name: ostr!("Input Dropdown Using Text By Label Exact"),
            description: ostr!("Input dropdown using text on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputDropdownUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("InputDropdownUsingIndexByLabelExact"),
            display_name: ostr!("Input Dropdown Using Index By Label Exact"),
            description: ostr!("Input dropdown using index on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputRadioUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("InputRadioUsingTextByLabelExact"),
            display_name: ostr!("Input Radio Using Text By Label Exact"),
            description: ostr!("Input radio using text on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::InputRadioUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("InputRadioUsingIndexByLabelExact"),
            display_name: ostr!("Input Radio Using Index By Label Exact"),
            description: ostr!("Input radio using index on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Label"),
                    description: ostr!("Label to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Input"),
                    description: ostr!("Input to send"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
            ],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToAnyIFrame"),
            display_name: ostr!("Get And Switch To Any IFrame"),
            description: ostr!("Find any IFrame within current frame, and switch into it."),
            args: vec![],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToAnyIFrame"),
            display_name: ostr!("Get And Switch To Any IFrame"),
            description: ostr!("Switch into parent IFrame"),
            args: vec![],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToRootIFrame"),
            display_name: ostr!("Get And Switch To Root IFrame"),
            description: ostr!("Switch to Root (top most) IFrame"),
            args: vec![],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::SetWindowDimension => FunctionMetadata {
            name: ostr!("SetWindowDimension"),
            display_name: ostr!("Set Window Dimension"),
            description: ostr!("Set window mode to windowed, and resize window"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Width"),
                    description: ostr!("Width of the window"),
                    arg_type: ostr!("number"),
                    default_value: ostr!("800"),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Height"),
                    description: ostr!("Height of the window"),
                    arg_type: ostr!("number"),
                    default_value: ostr!("600"),
                },
            ],
            return_type: ostr!("void"),
        },
    }
}

pub enum BuiltinPkgVariables {}
