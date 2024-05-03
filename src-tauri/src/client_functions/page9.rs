use crate::tasks::{task8and9, util};

#[tauri::command]
pub fn get_pcnf(function: &str) -> Result<String, &'static str> {
    let func = util::BooleanFunction::from(function)?;
    Ok(task8and9::get_pcnf(&func))
}