use crate::tasks::util;

#[tauri::command]
pub fn get_remind_function(func: &str, n: &str, value: bool) -> Result<String, &'static str> {
    let Ok(n) = n.parse::<u8>() else {
        return Err("Ошибка парсинга! Ожидалось значение типа u8");
    };
    if n == 0 {
        return Err("Аргумент должен быть больше нуля");
    }
    
    let func = util::BooleanFunction::from(func)?;

    if n > func.get_count_args() {
        return Err("Превышено максимальное допустимое значение");
    }

    
    let n = func.get_count_args() - n;

    func.remainde_boolean_function(n, value)
}