mod tasks {
    pub mod util;
    pub mod task3;
    pub mod task4;
    pub mod task5;
    pub mod task6and7;
    pub mod task8and9;
    pub mod task10and11;
    pub mod task12;
}

mod client_functions {
    pub mod page1;
    pub mod page2;
    pub mod page3;
    pub mod page4;
    pub mod page5;
    pub mod page6;
    pub mod page7;
    pub mod page8;
    pub mod page9;
    pub mod page10;
    pub mod page11;
    pub mod page12;
    pub mod client_utils;
}

mod parser;

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            client_functions::page1::get_random_bool_func,
            client_functions::page2::get_remind_function,
            client_functions::page3::get_func_from_remainde,
            client_functions::page4::get_name_funcs,
            client_functions::page4::get_random_vector_func,
            client_functions::page5::get_dummy_variable,
            client_functions::page6::check_dnf,
            client_functions::page7::check_cnf,
            client_functions::page8::get_pdnf,
            client_functions::page9::get_pcnf,
            client_functions::page10::match_function_to_precomplete_classes,
            client_functions::page11::match_functions_to_precomplete_classes,
            client_functions::page12::get_expression_dnf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}