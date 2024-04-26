use crate::tasks::task4;

#[tauri::command]
pub fn get_name_funcs() -> Vec<&'static str> {
    task4::get_name_funcs()
}

#[tauri::command]
pub fn get_random_vector_func() -> (&'static str, usize) {
    task4::get_random_bynary_boolean_func()
}

#[tauri::command]
pub fn check_equal_func_by_index(func: &str, index: usize) -> bool {
    task4::check_equal_binary_boolean_func(func, index)
}