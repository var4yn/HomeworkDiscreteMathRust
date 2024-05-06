use std::collections::HashMap;

use super::util;
use crate::parser;


// Игра. ДНФ. Система предлагает вектор функции. Пользователь вводит ДНФ. 
// Система определяет правильно или нет введена ДНФ.

/// На вход булевая функция и expression пользователя
/// Если функция не может иметь ДНФ, то она вернет ошибку
pub fn check_dnf(
    func: util::BooleanFunction,
    expression: &str
) -> Result<bool, String> {
    if !func.have_dnf() {
        return Err("The function does not have dnf".to_string())
    }
    // получаем выражение
    let expression = parser::parse::get_ast_tree(expression)?;

    if !parser::validate::is_dnf(&expression) {
        return Err("This is not DNF".to_string());
    }
    brute_func_vals(expression, func)
}

// Игра. КНФ. Система предлагает вектор функции.
// Пользователь вводит КНФ. Система определяет правильно или нет введена КНФ.

/// На вход булевая функция и expression пользователя
/// Если функция не может иметь КНФ, то она вернет ошибку
pub fn check_cnf(
    func: util::BooleanFunction,
    expression: &str
) -> Result<bool, String> {
    if !func.have_cnf() {
        return Err("The function does not have cnf".to_string());
    }
    // получаем выражение
    let expression = parser::parse::get_ast_tree(expression)?;

    if !parser::validate::is_cnf(&expression) {
        return Err("This is not CNF".to_string());
    }
    brute_func_vals(expression, func)
}



/// Перебор значений функции
fn brute_func_vals(
    expression: parser::Expression,
    func: util::BooleanFunction
) -> Result<bool, String> {

    // создание переменных
    let mut vars = HashMap::new();
    // пробег по вектору функции
    for (vc, func_val) in &func {
        // установка значений для переменных на определенном двоичном наборе
        for (i, &val) in vc.iter().enumerate() {
            vars.insert(format!("x{}", i + 1), val);
        }

        match expression.evaluate(&vars) {
            Ok(cur) => {
                // если значение функции не совпало со значением выражения
                // то возвращаем false
                if func_val != cur {
                    return Ok(false);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(true)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dnf_0000() {
        let func = util::BooleanFunction::from("0000").unwrap();
        let r = check_dnf(func, "");
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn test_cnf_1111() {
        let func = util::BooleanFunction::from("1111").unwrap();
        let r = check_cnf(func, "");
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn test_dnf() {
        let func_vector = "00010111";
        let func = util::BooleanFunction::from(func_vector).unwrap();
        let r = check_dnf(func.clone(), "-x1&x2&x3 v x1&-x2&x3 v x1&x2&-x3 v x1&x2&x3").unwrap();
        
        assert_eq!(r, true);

        let r = check_dnf(func, "-x1&x2&x3 v x1&-x2&x3 v x1&x2&-x3 v x1&x2&-x3").unwrap();
        
        assert_eq!(r, false);
    }

    #[test]
    fn test_cnf() {
        let func_vector = "00010111";
        let func = util::BooleanFunction::from(func_vector).unwrap();
        let r = check_cnf(func.clone(), " (x1 v x2 v x3) & (x1 v x2 v -x3) & ( x1 v -x2 v x3 ) & ( -x1 v x2 v x3 ) ").unwrap();

        assert_eq!(r, true);

        let r = check_cnf(func, " (x1 v x2 v x3) & (x1 v x2 v -x3) & ( x1 v -x2 v -x3 ) & ( -x1 v x2 v x3 ) ").unwrap();

        assert_eq!(r, false);

    }

    #[test]
    fn test_sim_dnf() {
        let func = "10110011";
        let func = util::BooleanFunction::from(func).unwrap();

        let r = check_dnf(func, "x2 v -x1&-x3").unwrap();

        assert_eq!(r, true);
    }

    #[test]
    fn test_not_expected_variable() {
        let func = "10110011";
        let func = util::BooleanFunction::from(func).unwrap();

        let Err(err) = check_dnf(func, "x2 v -x1&-x3 v x10") else {
            panic!("expected error!");
        };
        assert_eq!(err.contains("x10"), true);
    }

}