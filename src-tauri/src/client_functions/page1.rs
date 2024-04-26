use crate::tasks::util;
use super::client_utils;

#[tauri::command]
pub fn get_random_bool_func(n: &str) -> Result<String, &'static str> {
    let n = client_utils::parse_u8(n)?;
    
    Ok(util::BooleanFunction::with_count_args(n).get_func().into())
}