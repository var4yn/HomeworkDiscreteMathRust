use crate::tasks::{task5, util};

#[tauri::command]
pub fn get_dummy_variable(function: &str) -> Result<Vec<bool>, &'static str> {
    let func = util::BooleanFunction::from(function)?;
        
    Ok(task5::get_dummy_variable(func))
}