use crate::tasks::util;
use super::client_utils;

#[tauri::command]
pub fn get_remind_function(func: &str, n: &str, value: bool) -> Result<String, &'static str> {    
    let func = util::BooleanFunction::from(func)?;

    let n = client_utils::convert_num_arg(&func, n)?;

    func.remainde_boolean_function(n, value)
}