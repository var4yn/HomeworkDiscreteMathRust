use crate::tasks::{task8and9, util};

#[tauri::command]
pub fn get_pdnf(function: &str) -> Result<String, &'static str> {
    let func = util::BooleanFunction::from(function)?;
    Ok(task8and9::get_pdnf(&func))
}