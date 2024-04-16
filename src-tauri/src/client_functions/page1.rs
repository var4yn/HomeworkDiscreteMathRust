use crate::tasks::util;

#[tauri::command]
pub fn get_random_bool_func(n: &str) -> Result<String, &'static str> {
    let Ok(n) = n.parse::<u8>() else {
        return Err("Ошибка парсинга! Ожидалось значение типа u8");
    };
    
    Ok(util::BooleanFunction::with_count_args(n).get_func().into())
}