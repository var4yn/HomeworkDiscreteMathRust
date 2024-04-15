use std::collections::HashMap;

use super::util;
use crate::parser;


// Игра. ДНФ. Система предлагает вектор функции. Пользователь вводит ДНФ. 
// Система определяет правильно или нет введена ДНФ.
pub mod task6 {
    use super::*;

    /// На вход булевая функция и expression пользователя
    pub fn check_dnf(
        func: util::BooleanFunction,
        expression: &str
    ) -> Result<bool, String> {
        // выражение
        let result_expression = parser::parse::get_ast_tree(expression);
    
        if let Err(e) = result_expression {
            return Err(e.to_string());
        }
        let expression = result_expression.unwrap();
        if !parser::validate::is_dnf(&expression) {
            return Err("This is not DNF".to_string());
        }
        brute_func_vals(expression, func)
    }
}

// Игра. КНФ. Система предлагает вектор функции.
// Пользователь вводит КНФ. Система определяет правильно или нет введена КНФ.
pub mod task7 {
    use super::*;

    /// На вход булевая функция и expression пользователя
    pub fn check_knf(
        func: util::BooleanFunction,
        expression: &str
    ) -> Result<bool, String> {
        // выражение
        let result_expression = parser::parse::get_ast_tree(expression);
    
        if let Err(e) = result_expression {
            return Err(e.to_string());
        }
        let expression = result_expression.unwrap();
        if !parser::validate::is_knf(&expression) {
            return Err("This is not KNF".to_string());
        }
        brute_func_vals(expression, func)
    }
}





// перебор значений функции
fn brute_func_vals(
    expression: parser::Expression,
    func: util::BooleanFunction
) -> Result<bool, String> {

    for (vc, func_val) in util::BooleanFunctionIterator::new(&func) {
        let mut vars = HashMap::new();
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

    /* старый код без итератора

    for (i, ch) in func.get_func().char_indices() {
        let mut vars = HashMap::new();
        
        let mut len = 1;
        for j in (0..func.get_count_args()).rev() {
            let var = format!("x{}", j + 1);
            let value = i / len & 1;
            len *= 2;
            vars.insert(var, value == 1usize);
        }

        // подстановка значений
        let func_val = ch == '1';
        match expression.evaluate(&vars) {
            Ok(cur) => {
                // если значение функции не совпало со значением выраэения
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
    */

    Ok(true)
}



#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_dnf() {
        let func_vector = "00010111";
        let func = util::BooleanFunction::from(func_vector).unwrap();
        let r = task6::check_dnf(func.clone(), "-x1&x2&x3 v x1&-x2&x3 v x1&x2&-x3 v x1&x2&x3").unwrap();
        
        assert_eq!(r, true);

        let r = task6::check_dnf(func, "-x1&x2&x3 v x1&-x2&x3 v x1&x2&-x3 v x1&x2&-x3").unwrap();
        
        assert_eq!(r, false);
    }

    #[test]
    fn test_knf() {
        let func_vector = "00010111";
        let func = util::BooleanFunction::from(func_vector).unwrap();
        let r = task7::check_knf(func.clone(), " (x1 v x2 v x3) & (x1 v x2 v -x3) & ( x1 v -x2 v x3 ) & ( -x1 v x2 v x3 ) ").unwrap();

        assert_eq!(r, true);

        let r = task7::check_knf(func, " (x1 v x2 v x3) & (x1 v x2 v -x3) & ( x1 v -x2 v -x3 ) & ( -x1 v x2 v x3 ) ").unwrap();

        assert_eq!(r, false);

    }

    #[test]
    fn test_sim_dnf() {
        let func = "10110011";
        let func = util::BooleanFunction::from(func).unwrap();

        let r = task6::check_dnf(func, "x2 v -x1&-x3").unwrap();

        assert_eq!(r, true);
        

    }

}