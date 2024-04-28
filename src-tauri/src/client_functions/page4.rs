use crate::tasks::task4;

#[tauri::command]
pub fn get_name_funcs() -> Vec<&'static str> {
    task4::get_name_funcs()
}

#[tauri::command]
pub fn get_random_vector_func() -> (&'static str, &'static str) {
    task4::get_random_bynary_boolean_func()
}