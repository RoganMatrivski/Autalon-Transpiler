use color_eyre::eyre::{bail, Report};
use std::str::FromStr;

use crate::builtin_package_definition::{self, BuiltinPkgFunctions};

fn get_katalon_default_fn_metadata<'a>(builtin_fn: BuiltinPkgFunctions) -> &'a str {
    match builtin_fn {
        BuiltinPkgFunctions::NavigateToUrl => r#"driver.navigate().to({arg1})"#,
        BuiltinPkgFunctions::GetElementByString => {
            r#"driverExt.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable()"#
        }
        BuiltinPkgFunctions::ClickElementByString => {
            r#"driverExt.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable().click()"#
        }
        BuiltinPkgFunctions::SendTextToElementByString => {
            r#"driverExt.getElement().byString({arg1}, {arg3}, {arg4}, {arg5}).untilElementInteractable().sendKeys({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIGetInputFromLabel => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetIFrameFromLabel => {
            r#"new ExtUIGetter(driver).getIFrameFromLabel({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetWindowFromLabel => {
            r#"new ExtUIGetter(driver).getWindowFromTitle({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetGroupFromLabel => {
            r#"new ExtUIGetter(driver).getGroupFromTitle({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIInputDateByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().date().sendText({arg2}, false)"#
        }
        BuiltinPkgFunctions::ExtUIInputHtmlByLabelExact => {
            r#"new ExtUIGetter(driver).getIFrameFromLabel({arg1}).shouldBe().htmlEditor().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputNumberTextboxByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().numberTextbox().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputTextboxByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().textbox().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputDropdownUsingTextByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().dropdown().selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputDropdownUsingIndexByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().dropdown().selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputRadioUsingTextByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().radio().selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputRadioUsingIndexByLabelExact => {
            r#"new ExtUIGetter(driver).getInputFromLabel({arg1}).shouldBe().radio().selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => {
            r#"driver = driverExt.waitUntilFrameLoads(By.xpath('//iframe')); driverExt = new WebDriverExtended(driver)"#
        }
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => {
            r#"driver = driver.switchTo().parentFrame(); driverExt = new WebDriverExtended(driver)"#
        }
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => {
            r#"driver = driver.switchTo().defaultContent(); driverExt = new WebDriverExtended(driver)"#
        }
        BuiltinPkgFunctions::SetWindowDimension => {
            r#"driverExt.setWindowDimension({arg1}, {arg2})"#
        }
        BuiltinPkgFunctions::MUIInputTextboxByLabelExact => {
            r#"new ReactMUIGetter(driver).getTextboxFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDateByLabelExact => {
            r#"new ReactMUIGetter(driver).getDateFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputTimeByLabelExact => {
            r#"new ReactMUIGetter(driver).getTimeFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputHtmlByLabelExact => {
            r#"new ReactMUIGetter(driver).getHTMLFromLabel({arg1}).clearText().sendRawText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDropdownUsingTextByLabelExact => {
            r#"new ReactMUIGetter(driver).getDropdownFromLabel({arg1}).selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDropdownUsingIndexByLabelExact => {
            r#"new ReactMUIGetter(driver).getDropdownFromLabel({arg1}).selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputRadioUsingTextByLabelExact => {
            r#"new ReactMUIGetter(driver).getRadioFromLabel({arg1}).selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputRadioUsingIndexByLabelExact => {
            r#"new ReactMUIGetter(driver).getRadioFromLabel({arg1}).selectElementOnIndex({arg2})"#
        }
        // fn_enum => unimplemented!(
        //     "Function {} from default package is currently unimplemented.",
        //     fn_enum
        // ),
    }
}

pub fn get_default_fn_template<'a>(name: &'a str, pkg: &'a str) -> Result<&'a str, Report> {
    // TODO: Remove hardcoded package alias switching
    let pkg = if pkg == "#" { "builtin" } else { pkg };

    let fn_template = match pkg {
        "builtin" => {
            let fn_enum = builtin_package_definition::BuiltinPkgFunctions::from_str(name)?;

            get_katalon_default_fn_metadata(fn_enum)
        }
        str => bail!("Package {} is currently unimplemented.", str),
    };

    Ok(fn_template)
}
