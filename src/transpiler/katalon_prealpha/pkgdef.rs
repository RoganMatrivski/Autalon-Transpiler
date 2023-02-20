use color_eyre::eyre::{bail, Report};
use std::str::FromStr;

use crate::builtin_package_definition::{self, BuiltinPkgFunctions};

fn get_katalon_default_fn_metadata<'a>(builtin_fn: BuiltinPkgFunctions) -> &'a str {
    match builtin_fn {
        BuiltinPkgFunctions::NavigateToUrl => r#"driver.navigate().to({arg1})"#,
        BuiltinPkgFunctions::GetElementByString => {
            r#"driver.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable()"#
        }
        BuiltinPkgFunctions::ClickElementByString => {
            r#"driver.getElement().byString({arg1}, {arg2}, {arg3}, {arg4}).untilElementInteractable().click()"#
        }
        BuiltinPkgFunctions::SendTextToElementByString => {
            r#"driver.getElement().byString({arg1}, {arg3}, {arg4}, {arg5}).untilElementInteractable().sendKeys({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIGetInputFromLabel => {
            r#"extUIGetter.getInputFromLabel({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetIFrameFromLabel => {
            r#"extUIGetter.getIFrameFromLabel({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetWindowFromLabel => {
            r#"extUIGetter.getWindowFromTitle({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIGetGroupFromLabel => {
            r#"extUIGetter.getGroupFromTitle({arg1})"#
        }
        BuiltinPkgFunctions::ExtUIInputDateByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().date().sendText({arg2}, false)"#
        }
        BuiltinPkgFunctions::ExtUIInputHtmlByLabelExact => {
            r#"extUIGetter.getIFrameFromLabel({arg1}).shouldBe().htmlEditor().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputNumberTextboxByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().numberTextbox().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputTextboxByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().textbox().sendText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputDropdownUsingTextByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().dropdown().selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputDropdownUsingIndexByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().dropdown().selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputRadioUsingTextByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().radio().selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::ExtUIInputRadioUsingIndexByLabelExact => {
            r#"extUIGetter.getInputFromLabel({arg1}).shouldBe().radio().selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::GetAndSwitchToAnyIFrame => {
            r#"driver = driver.waitUntilFrameLoads(By.xpath('//iframe')); driver = new Webdriverended(driver)"#
        }
        BuiltinPkgFunctions::GetAndSwitchToParentIFrame => {
            r#"driver = driver.switchTo().parentFrame(); driver = new Webdriverended(driver)"#
        }
        BuiltinPkgFunctions::GetAndSwitchToRootIFrame => {
            r#"driver = driver.switchTo().defaultContent(); driver = new Webdriverended(driver)"#
        }
        BuiltinPkgFunctions::SetWindowDimension => {
            r#"driver.setWindowDimension({arg1}, {arg2})"#
        }
        BuiltinPkgFunctions::MUIInputTextboxByLabelExact => {
            r#"reactMUIGetter.getTextboxFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDateByLabelExact => {
            r#"reactMUIGetter.getDateFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputTimeByLabelExact => {
            r#"reactMUIGetter.getTimeFromLabel({arg1}).sendText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputHtmlByLabelExact => {
            r#"reactMUIGetter.getHTMLFromLabel({arg1}).clearText().sendRawText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDropdownUsingTextByLabelExact => {
            r#"reactMUIGetter.getDropdownFromLabel({arg1}).selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputDropdownUsingIndexByLabelExact => {
            r#"reactMUIGetter.getDropdownFromLabel({arg1}).selectElementOnIndex({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputRadioUsingTextByLabelExact => {
            r#"reactMUIGetter.getRadioFromLabel({arg1}).selectElementFromText({arg2})"#
        }
        BuiltinPkgFunctions::MUIInputRadioUsingIndexByLabelExact => {
            r#"reactMUIGetter.getRadioFromLabel({arg1}).selectElementOnIndex({arg2})"#
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
