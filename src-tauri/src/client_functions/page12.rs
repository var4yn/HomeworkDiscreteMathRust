use crate::tasks::{task12, util};

#[tauri::command]
pub fn get_expression_dnf(function: &str) -> Result<String, &'static str> {
    let func = util::BooleanFunction::from(function)?;
    Ok(task12::get_expression_dnf_with_qms(func))
}