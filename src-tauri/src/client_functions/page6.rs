use crate::tasks::{task6and7, util};

#[tauri::command]
pub fn check_dnf(function: &str, expression: &str) -> Result<bool, String> {
    let func = util::BooleanFunction::from(function)?;
    if !func.have_dnf() {
        return Err("Эта функция не имеет ДНФ".to_string());
    }
    task6and7::check_dnf(func, expression)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_in_boolean_function_cast() {
        assert_eq!(check_dnf("10101", ""), Err("Function length must be a power of two.".to_string()));
        assert_eq!(check_dnf("1010sdf", ""), Err("Function must be consist by 0 or 1.".to_string()));
    }
}