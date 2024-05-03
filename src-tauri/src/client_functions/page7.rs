use crate::tasks::{task6and7, util};

#[tauri::command]
pub fn check_knf(function: &str, expression: &str) -> Result<bool, String> {
    let func = util::BooleanFunction::from(function)?;
    if !func.have_knf() {
        return Err("Эта функция не имеет КНФ".to_string());
    }
    task6and7::check_knf(func, expression)
}