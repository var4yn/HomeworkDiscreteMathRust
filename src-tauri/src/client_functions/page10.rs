use crate::tasks::{task10and11, util};

#[tauri::command]
pub fn match_function_to_precomplete_classes(function: &str) -> Result<[ bool;5 ], &'static str> {
    let func = util::BooleanFunction::from(function)?;
    Ok(task10and11::match_function_to_precomplete_classes(&func))
}