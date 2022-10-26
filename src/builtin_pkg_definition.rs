use serde::Serialize;

use strum_macros::{Display, EnumIter, EnumString};

use crate::ostr;

#[allow(dead_code)]
enum ArgType {
    String,
    Number,
    Boolean,
    ByOption,
}

#[derive(PartialEq, EnumString, EnumIter, Display)]
pub enum BuiltinPkgFunctions {
    NavigateToUrl,
    GetElementByString,
    ClickElementByString,
    SendTextToElementByString,
    GetAndSwitchToAnyIFrame,
    GetAndSwitchToParentIFrame,
    GetAndSwitchToRootIFrame,

    SetWindowDimension,

    // ExtUI funcs
    ExtUIGetInputFromLabel,
    ExtUIGetIFrameFromLabel,
    ExtUIGetWindowFromLabel,
    ExtUIGetGroupFromLabel,
    ExtUIInputDateByLabelExact,
    ExtUIInputHtmlByLabelExact,
    ExtUIInputNumberTextboxByLabelExact,
    ExtUIInputTextboxByLabelExact,
    ExtUIInputDropdownUsingTextByLabelExact,
    ExtUIInputDropdownUsingIndexByLabelExact,
    ExtUIInputRadioUsingTextByLabelExact,
    ExtUIInputRadioUsingIndexByLabelExact,

    // MUI funcs
    MUIInputTextboxByLabelExact,
    MUIInputDateByLabelExact,
    MUIInputTimeByLabelExact,
    MUIInputHtmlByLabelExact,
    MUIInputDropdownUsingTextByLabelExact,
    MUIInputDropdownUsingIndexByLabelExact,
    MUIInputRadioUsingTextByLabelExact,
    MUIInputRadioUsingIndexByLabelExact,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionArgsMetadata {
    pub display_name: String,
    pub description: String,
    pub arg_type: String,
    pub default_value: String,
}

#[derive(Serialize, Display)]
pub enum TargetUI {
    Any,
    Ext,
    MUI,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionMetadata {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub target_ui: TargetUI,
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
            target_ui: TargetUI::Any,
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
                    default_value: ostr!("true"),
                },
            ],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
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
                    default_value: ostr!("true"),
                },
            ],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
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
                    default_value: ostr!("true"),
                },
            ],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
        },

        BuiltinPkgFunctions::ExtUIGetInputFromLabel => FunctionMetadata {
            name: ostr!("ExtUIGetInputFromLabel"),
            display_name: ostr!("Get Input From Label"),
            description: ostr!("Get input element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::ExtUIGetIFrameFromLabel => FunctionMetadata {
            name: ostr!("ExtUIGetIFrameFromLabel"),
            display_name: ostr!("Get IFrame From Label"),
            description: ostr!("Get IFrame element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIGetWindowFromLabel => FunctionMetadata {
            name: ostr!("ExtUIGetWindowFromLabel"),
            display_name: ostr!("Get Window From Label"),
            description: ostr!("Get window element from title text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIGetGroupFromLabel => FunctionMetadata {
            name: ostr!("GetGroupFromLabelExtUI"),
            display_name: ostr!("Get Group From Label"),
            description: ostr!("Get group element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ostr!("Text"),
                description: ostr!("Text label to find"),
                arg_type: ostr!("string"),
                default_value: ostr!(""),
            }],
            return_type: ostr!("void"),
            target_ui: TargetUI::Ext,
        },

        BuiltinPkgFunctions::ExtUIInputDateByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputDateByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputHtmlByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputHtmlByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputNumberTextboxByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputNumberTextboxByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputTextboxByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputTextboxByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputDropdownUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputDropdownUsingTextByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputDropdownUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputDropdownUsingIndexByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputRadioUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputRadioUsingTextByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputRadioUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("ExtUIInputRadioUsingIndexByLabelExact"),
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
            target_ui: TargetUI::Ext,
        },

        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToAnyIFrame"),
            display_name: ostr!("Get And Switch To Any IFrame"),
            description: ostr!("Find any IFrame within current frame, and switch into it."),
            args: vec![],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToAnyIFrame"),
            display_name: ostr!("Get And Switch To Any IFrame"),
            description: ostr!("Switch into parent IFrame"),
            args: vec![],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => FunctionMetadata {
            name: ostr!("GetAndSwitchToRootIFrame"),
            display_name: ostr!("Get And Switch To Root IFrame"),
            description: ostr!("Switch to Root (top most) IFrame"),
            args: vec![],
            return_type: ostr!("void"),
            target_ui: TargetUI::Any,
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
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::MUIInputTextboxByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputTextboxByLabelExact"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDropdownUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputDropdownUsingTextByLabelExact"),
            display_name: ostr!("Input Dropdown By Label Exact"),
            description: ostr!("Input dropdown on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDropdownUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputDropdownUsingIndexByLabelExact"),
            display_name: ostr!("Input Dropdown By Label Exact"),
            description: ostr!("Input dropdown on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputRadioUsingTextByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputRadioByLabelExact"),
            display_name: ostr!("Input Radio Using Text By Label Exact"),
            description: ostr!("Input Radio on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputRadioUsingIndexByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputRadioUsingIndexByLabelExact"),
            display_name: ostr!("Input Radio Using Index By Label Exact"),
            description: ostr!("Input Radio on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDateByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputDateByLabelExact"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputTimeByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputTimeByLabelExact"),
            display_name: ostr!("Input Time By Label Exact"),
            description: ostr!("Input time on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputHtmlByLabelExact => FunctionMetadata {
            name: ostr!("MUIInputHtmlByLabelExact"),
            display_name: ostr!("Input HTML By Label Exact"),
            description: ostr!("Input HTML on form by label form"),
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
            target_ui: TargetUI::MUI,
        },
    }
}

#[allow(dead_code)]
pub enum BuiltinPkgVariables {}
