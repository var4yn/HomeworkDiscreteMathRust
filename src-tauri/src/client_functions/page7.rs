use crate::tasks::{task6and7, util};

#[tauri::command]
pub fn check_cnf(function: &str, expression: &str) -> Result<bool, String> {
    let func = util::BooleanFunction::from(function)?;
    if !func.have_cnf() {
        return Err("Эта функция не имеет КНФ".to_string());
    }
    task6and7::check_cnf(func, expression)
}