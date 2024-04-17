use crate::tasks::{task3, util};
use super::client_utils;


#[tauri::command]
pub fn get_func_from_remainde(func0: &str, func1: &str, n: &str) -> Result<String, &'static str> {
    let mut func = func0.to_owned();
    func.push_str(func1);
    
    let func = util::BooleanFunction::from(func)?;
    let n = client_utils::convert_num_arg(&func, n)?;

    Ok(task3::boolean_function_from_remainde_func(func0, func1, n)?.get_func().to_string())
}