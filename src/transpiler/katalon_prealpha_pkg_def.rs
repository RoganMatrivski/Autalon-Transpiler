use std::str::FromStr;

use crate::{
    builtin_pkg_definition::{self, BuiltinPkgFunctions},
    ostr,
};

fn get_katalon_default_fn_metadata(builtin_fn: &BuiltinPkgFunctions) -> String {
    match builtin_fn {
        BuiltinPkgFunctions::NavigateToUrl => ostr!(r#"driver.navigate().to({arg1})"#),
        BuiltinPkgFunctions::GetElementByString => ostr!(
            r#"driverExt.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable()"#
        ),
        BuiltinPkgFunctions::ClickElementByString => ostr!(
            r#"driverExt.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable().click()"#
        ),
        BuiltinPkgFunctions::SendTextToElementByString => ostr!(
            r#"driverExt.getElement().byString({arg1}, {arg3}, {arg4}, {arg5}).untilElementInteractable().sendKeys({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIGetInputFromLabel => {
            ostr!(r#"new ExtUIGetter(driver).getInputFromLabel({arg1})"#)
        }
        BuiltinPkgFunctions::ExtUIGetIFrameFromLabel => {
            ostr!(r#"new ExtUIGetter(driver).getIFrameFromLabel({arg1})"#)
        }
        BuiltinPkgFunctions::ExtUIGetWindowFromLabel => {
            ostr!(r#"new ExtUIGetter(driver).getWindowFromTitle({arg1})"#)
        }
        BuiltinPkgFunctions::ExtUIGetGroupFromLabel => {
            ostr!(r#"new ExtUIGetter(driver).getGroupFromTitle({arg1})"#)
        }
        BuiltinPkgFunctions::ExtUIInputDateByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().date().sendText({arg2}, false)"#
        ),
        BuiltinPkgFunctions::ExtUIInputHtmlByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getIFrameFromLabel({arg1}).shouldBe().htmlEditor().sendText({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputNumberTextboxByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().numberTextbox().sendText({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputTextboxByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().textbox().sendText({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputDropdownUsingTextByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().dropdown().selectElementFromText({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputDropdownUsingIndexByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().dropdown().selectElementOnIndex({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputRadioUsingTextByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().radio().selectElementFromText({arg2})"#
        ),
        BuiltinPkgFunctions::ExtUIInputRadioUsingIndexByLabelExact => ostr!(
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().radio().selectElementOnIndex({arg2})"#
        ),
        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => ostr!(
            r#"driver = driverExt.waitUntilFrameLoads(By.xpath('//iframe')); driverExt = new WebDriverExtended(driver)"#
        ),
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => ostr!(
            r#"driver = driver.switchTo().parentFrame(); driverExt = new WebDriverExtended(driver)"#
        ),
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => ostr!(
            r#"driver = driver.switchTo().defaultContent(); driverExt = new WebDriverExtended(driver)"#
        ),
        BuiltinPkgFunctions::SetWindowDimension => {
            ostr!(r#"driverExt.setWindowDimension({arg1}, {arg2})"#)
        }
        BuiltinPkgFunctions::MUIInputTextboxByLabelExact => {
            ostr!(r#"new ReactMUIGetter(driver).getTextboxFromLabel({arg1}).sendText({arg2})"#)
        }
        // fn_enum => unimplemented!(
        //     "Function {} from default package is currently unimplemented.",
        //     fn_enum
        // ),
    }
}

pub fn get_default_fn_template(name: &str, pkg: &str) -> String {
    // TODO: Remove hardcoded package alias switching
    let pkg = if pkg == "#" { "builtin" } else { pkg };

    let fn_template = match pkg {
        "builtin" => {
            let fn_enum = match builtin_pkg_definition::BuiltinPkgFunctions::from_str(name) {
                Ok(res) => res,
                Err(err) => panic!("{}", err.to_string()),
            };

            get_katalon_default_fn_metadata(&fn_enum)
        }
        str => unimplemented!("Package {} is currently unimplemented.", str),
    };

    fn_template
}
