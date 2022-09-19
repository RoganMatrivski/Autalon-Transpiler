use serde::Serialize;

use strum_macros::{Display, EnumIter, EnumString};

use crate::ostr;

enum ArgType {
    String,
    Number,
    Boolean,
    ByOption,
}

#[derive(PartialEq, EnumString, EnumIter, Display)]
pub enum BuiltinPkgFunctions {
    NavigateToUrl,
    GetElementByText,
    GetElementByTextExact,
    GetElementByString,
    GetElementByStringExact,
    ClickElementByText,
    ClickElementByTextExact,
    ClickElementByString,
    ClickElementByStringExact,
    SendTextToElementByString,
    SendTextToElementByStringExact,
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

        BuiltinPkgFunctions::GetElementByText => FunctionMetadata {
            name: ostr!("GetElementByText"),
            display_name: ostr!("Get Element By Text"),
            description: ostr!("Get element by text specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Text"),
                    description: ostr!("Text to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetElementByTextExact => FunctionMetadata {
            name: ostr!("GetElementByTextExact"),
            display_name: ostr!("Get Element By Exact Text"),
            description: ostr!("Get element by exact text specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Text"),
                    description: ostr!("Text to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
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
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::GetElementByStringExact => FunctionMetadata {
            name: ostr!("GetElementByStringExact"),
            display_name: ostr!("Get Element By Exact String"),
            description: ostr!("Get element by exact string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },

        BuiltinPkgFunctions::ClickElementByText => FunctionMetadata {
            name: ostr!("ClickElementByText"),
            display_name: ostr!("Click Element By Text"),
            description: ostr!("Click element by text specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Text"),
                    description: ostr!("Text to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::ClickElementByTextExact => FunctionMetadata {
            name: ostr!("ClickElementByTextExact"),
            display_name: ostr!("Click Element By Exact Text"),
            description: ostr!("Click element by exact text specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("Text"),
                    description: ostr!("Text to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
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
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::ClickElementByStringExact => FunctionMetadata {
            name: ostr!("ClickElementByStringExact"),
            display_name: ostr!("Click Element By Exact String"),
            description: ostr!("Click element by exact string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
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
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
                },
            ],
            return_type: ostr!("void"),
        },
        BuiltinPkgFunctions::SendTextToElementByStringExact => FunctionMetadata {
            name: ostr!("SendTextToElementByStringExact"),
            display_name: ostr!("Send text to Element By Exact String"),
            description: ostr!("Send text to element by exact string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ostr!("String"),
                    description: ostr!("String to find"),
                    arg_type: ostr!("string"),
                    default_value: ostr!(""),
                },
                FunctionArgsMetadata {
                    display_name: ostr!("Tag"),
                    description: ostr!("Tag to match for the element"),
                    arg_type: ostr!("string"),
                    default_value: ostr!("*"),
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
    }
}

pub enum BuiltinPkgVariables {}
