use serde::Serialize;

use strum_macros::{Display, EnumIter, EnumString};

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
    pub display_name: &'static str,
    pub description: &'static str,
    pub arg_type: &'static str,
    pub default_value: &'static str,
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
    pub name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub target_ui: TargetUI,
    pub args: Vec<FunctionArgsMetadata>,
    pub return_type: &'static str,
}

pub fn get_fn_metadata(builtin_fn: &BuiltinPkgFunctions) -> FunctionMetadata {
    match builtin_fn {
        BuiltinPkgFunctions::NavigateToUrl => FunctionMetadata {
            name: ("NavigateToUrl"),
            display_name: ("Navigate To Url"),
            description: ("Navigate current page to Url"),
            args: vec![FunctionArgsMetadata {
                display_name: ("URL"),
                description: ("URL to navigate to"),
                arg_type: ("string"),
                default_value: ("https://www.google.com"),
            }],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },

        BuiltinPkgFunctions::GetElementByString => FunctionMetadata {
            name: ("GetElementByString"),
            display_name: ("Get Element By String"),
            description: ("Get element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("String"),
                    description: ("String to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("By Option"),
                    description: ("The attribute to match with the string"),
                    arg_type: ("byoption"),
                    default_value: ("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ("Tag"),
                    description: ("Tag to match for the element"),
                    arg_type: ("string"),
                    default_value: ("*"),
                },
                FunctionArgsMetadata {
                    display_name: ("Exact Match"),
                    description: ("Match the whole string"),
                    arg_type: ("bool"),
                    default_value: ("true"),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::ClickElementByString => FunctionMetadata {
            name: ("ClickElementByString"),
            display_name: ("Click Element By String"),
            description: ("Click element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("String"),
                    description: ("String to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("By Option"),
                    description: ("The attribute to match with the string"),
                    arg_type: ("byoption"),
                    default_value: ("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ("Tag"),
                    description: ("Tag to match for the element"),
                    arg_type: ("string"),
                    default_value: ("*"),
                },
                FunctionArgsMetadata {
                    display_name: ("Exact Match"),
                    description: ("Match the whole string"),
                    arg_type: ("bool"),
                    default_value: ("true"),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },

        BuiltinPkgFunctions::SendTextToElementByString => FunctionMetadata {
            name: ("SendTextToElementByString"),
            display_name: ("Send text to Element By String"),
            description: ("Send text to element by string specified"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("String"),
                    description: ("String to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("String to input"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("By Option"),
                    description: ("The attribute to match with the string"),
                    arg_type: ("byoption"),
                    default_value: ("ByOption.Text"),
                },
                FunctionArgsMetadata {
                    display_name: ("Tag"),
                    description: ("Tag to match for the element"),
                    arg_type: ("string"),
                    default_value: ("*"),
                },
                FunctionArgsMetadata {
                    display_name: ("Exact Match"),
                    description: ("Match the whole string"),
                    arg_type: ("bool"),
                    default_value: ("true"),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },

        BuiltinPkgFunctions::ExtUIGetInputFromLabel => FunctionMetadata {
            name: ("ExtUIGetInputFromLabel"),
            display_name: ("Get Input From Label"),
            description: ("Get input element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ("Text"),
                description: ("Text label to find"),
                arg_type: ("string"),
                default_value: (""),
            }],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::ExtUIGetIFrameFromLabel => FunctionMetadata {
            name: ("ExtUIGetIFrameFromLabel"),
            display_name: ("Get IFrame From Label"),
            description: ("Get IFrame element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ("Text"),
                description: ("Text label to find"),
                arg_type: ("string"),
                default_value: (""),
            }],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIGetWindowFromLabel => FunctionMetadata {
            name: ("ExtUIGetWindowFromLabel"),
            display_name: ("Get Window From Label"),
            description: ("Get window element from title text"),
            args: vec![FunctionArgsMetadata {
                display_name: ("Text"),
                description: ("Text label to find"),
                arg_type: ("string"),
                default_value: (""),
            }],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIGetGroupFromLabel => FunctionMetadata {
            name: ("GetGroupFromLabelExtUI"),
            display_name: ("Get Group From Label"),
            description: ("Get group element from label text"),
            args: vec![FunctionArgsMetadata {
                display_name: ("Text"),
                description: ("Text label to find"),
                arg_type: ("string"),
                default_value: (""),
            }],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },

        BuiltinPkgFunctions::ExtUIInputDateByLabelExact => FunctionMetadata {
            name: ("ExtUIInputDateByLabelExact"),
            display_name: ("Input Date By Label Exact"),
            description: ("Input date on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputHtmlByLabelExact => FunctionMetadata {
            name: ("ExtUIInputHtmlByLabelExact"),
            display_name: ("Input Html By Label Exact"),
            description: ("Input html on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputNumberTextboxByLabelExact => FunctionMetadata {
            name: ("ExtUIInputNumberTextboxByLabelExact"),
            display_name: ("Input Number Textbox By Label Exact"),
            description: ("Input number textbox on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputTextboxByLabelExact => FunctionMetadata {
            name: ("ExtUIInputTextboxByLabelExact"),
            display_name: ("Input Textbox By Label Exact"),
            description: ("Input textbox on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputDropdownUsingTextByLabelExact => FunctionMetadata {
            name: ("ExtUIInputDropdownUsingTextByLabelExact"),
            display_name: ("Input Dropdown Using Text By Label Exact"),
            description: ("Input dropdown using text on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputDropdownUsingIndexByLabelExact => FunctionMetadata {
            name: ("ExtUIInputDropdownUsingIndexByLabelExact"),
            display_name: ("Input Dropdown Using Index By Label Exact"),
            description: ("Input dropdown using index on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputRadioUsingTextByLabelExact => FunctionMetadata {
            name: ("ExtUIInputRadioUsingTextByLabelExact"),
            display_name: ("Input Radio Using Text By Label Exact"),
            description: ("Input radio using text on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },
        BuiltinPkgFunctions::ExtUIInputRadioUsingIndexByLabelExact => FunctionMetadata {
            name: ("ExtUIInputRadioUsingIndexByLabelExact"),
            display_name: ("Input Radio Using Index By Label Exact"),
            description: ("Input radio using index on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Ext,
        },

        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => FunctionMetadata {
            name: ("GetAndSwitchToAnyIFrame"),
            display_name: ("Get And Switch To Any IFrame"),
            description: ("Find any IFrame within current frame, and switch into it."),
            args: vec![],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => FunctionMetadata {
            name: ("GetAndSwitchToParentIFrame"),
            display_name: ("Get And Switch To Parent IFrame"),
            description: ("Switch into parent IFrame"),
            args: vec![],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => FunctionMetadata {
            name: ("GetAndSwitchToRootIFrame"),
            display_name: ("Get And Switch To Root IFrame"),
            description: ("Switch to Root (top most) IFrame"),
            args: vec![],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },

        BuiltinPkgFunctions::SetWindowDimension => FunctionMetadata {
            name: ("SetWindowDimension"),
            display_name: ("Set Window Dimension"),
            description: ("Set window mode to windowed, and resize window"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Width"),
                    description: ("Width of the window"),
                    arg_type: ("number"),
                    default_value: ("800"),
                },
                FunctionArgsMetadata {
                    display_name: ("Height"),
                    description: ("Height of the window"),
                    arg_type: ("number"),
                    default_value: ("600"),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::Any,
        },
        BuiltinPkgFunctions::MUIInputTextboxByLabelExact => FunctionMetadata {
            name: ("MUIInputTextboxByLabelExact"),
            display_name: ("Input Textbox By Label Exact"),
            description: ("Input textbox on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDropdownUsingTextByLabelExact => FunctionMetadata {
            name: ("MUIInputDropdownUsingTextByLabelExact"),
            display_name: ("Input Dropdown By Label Exact"),
            description: ("Input dropdown on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDropdownUsingIndexByLabelExact => FunctionMetadata {
            name: ("MUIInputDropdownUsingIndexByLabelExact"),
            display_name: ("Input Dropdown By Label Exact"),
            description: ("Input dropdown on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputRadioUsingTextByLabelExact => FunctionMetadata {
            name: ("MUIInputRadioByLabelExact"),
            display_name: ("Input Radio Using Text By Label Exact"),
            description: ("Input Radio on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputRadioUsingIndexByLabelExact => FunctionMetadata {
            name: ("MUIInputRadioUsingIndexByLabelExact"),
            display_name: ("Input Radio Using Index By Label Exact"),
            description: ("Input Radio on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputDateByLabelExact => FunctionMetadata {
            name: ("MUIInputDateByLabelExact"),
            display_name: ("Input Date By Label Exact"),
            description: ("Input date on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputTimeByLabelExact => FunctionMetadata {
            name: ("MUIInputTimeByLabelExact"),
            display_name: ("Input Time By Label Exact"),
            description: ("Input time on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
        BuiltinPkgFunctions::MUIInputHtmlByLabelExact => FunctionMetadata {
            name: ("MUIInputHtmlByLabelExact"),
            display_name: ("Input HTML By Label Exact"),
            description: ("Input HTML on form by label form"),
            args: vec![
                FunctionArgsMetadata {
                    display_name: ("Label"),
                    description: ("Label to find"),
                    arg_type: ("string"),
                    default_value: (""),
                },
                FunctionArgsMetadata {
                    display_name: ("Input"),
                    description: ("Input to send"),
                    arg_type: ("string"),
                    default_value: (""),
                },
            ],
            return_type: ("void"),
            target_ui: TargetUI::MUI,
        },
    }
}

#[allow(dead_code)]
pub enum BuiltinPkgVariables {}
