// Пользователь вводит вектор функции. Система строит СДНФ.


// алгоритм построения СДНФ :
// пропускаем строку, где функция принимает значение 0
// возвращаем ( x1 & x2 & x3 ), при которых функция принимает значение 1, и делаем отрицание переменных, у которых значение 0

use super::util;

// спорт прога стайл
fn get_expression(
    func: &util::BooleanFunction,
    func_value_skip: bool, // значение функции, которое скипаем
) -> String {
    let mut expr = String::new();
    let operator = vec!['&', 'v'];
    let j = func_value_skip as usize;

    for (current_vals, f_val) in func {
        if f_val == func_value_skip {
            continue;
        }
        let mut line = String::new();
        // если выражение непустое, то добавить оператор
        if !expr.is_empty() {
            line.push(' ');
            line.push(operator[j ^ 1]);
            line.push(' ');
        }
        // пробег по текущему двоичному набору
        for (i, &value) in current_vals.iter().enumerate() {
            if i == 0 {
                line.push('(');
            }
            // добавление переменной в выражение
            if value == !func_value_skip {
                line.push_str(&format!("x{}", i + 1));
            } else {
                line.push_str(&format!("-x{}", i + 1));
            }
            // добавление закрывающей скобки в конъюнкт/дизъюнкт
            if i == current_vals.len() - 1 {
                line.push(')');
            } else {
                // поставить оператор внутри конъюнкта/дизъюнкта
                line.push(operator[j]);
            }
        }
        expr.push_str(&line);
    }

    expr
}

pub fn get_pdnf(
    func: &util::BooleanFunction
) -> String {
    get_expression(func, false)
}

pub fn get_pcnf(
    func: &util::BooleanFunction
) -> String {
    get_expression(func, true)
}



#[cfg(test)]
mod tests {
    use crate::tasks::task6and7;

    use super::*;

    #[test]
    fn test_pdnf() {
        let func = util::BooleanFunction::from("00010111").unwrap();
        let expr = get_pdnf(&func);
        //println!("{}", expr);

        let r = task6and7::check_dnf(func, &expr).unwrap();

        assert_eq!(r, true);
    }

    #[test]
    fn test_pcnf() {
        let func = util::BooleanFunction::from("00010111").unwrap();
        let expr = get_pcnf(&func);
        println!("{}", expr);

        let r = task6and7::check_cnf(func, &expr).unwrap();

        assert_eq!(r, true);
    }

    #[test]
    fn test_random() {

        let func = util::BooleanFunction::with_count_args(4);
        let expr = get_pdnf(&func);

        let r = task6and7::check_dnf(func, &expr).unwrap();
        assert_eq!(r, true);


        let func = util::BooleanFunction::with_count_args(4);
        let expr = get_pcnf(&func);

        let r = task6and7::check_cnf(func, &expr).unwrap();
        assert_eq!(r, true);

    }


}