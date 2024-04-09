use super::util;

/// Игра. Предполные классы б.ф. Система предлагает вектор функции. 
/// Пользователь должен выбрать предполные классы, которым эта функция принадлежит.
/// Система определяет правильно выбраны классы или нет.
pub mod task10 {
    //
}


/// Игра. Полные системы б.ф. Система предлагает набор векторов функций.
/// Пользователь определяет полным или нет является набор функций. Если система б.ф. неполна, то пользователь
/// должен указать замкнутый класс, которому набор функций принадлежит.
pub mod task11 {
    //
}


// необходимо сделать проверку на принадлежность к множествам:
// T1, T0, M, L, S.

/// Проверка, что на всех нулях функция возвращает ноль.
fn is_t0(
    func: &util::BooleanFunction
) -> bool {
    func.get_func().chars().next() == Some('0')
}

/// Проверка, что на всех единицах функция возвращает единицу.
fn is_t1(
    func: &util::BooleanFunction
) -> bool {
    func.get_func().chars().last() == Some('1')
}

// основная сложность проверить на M и на L.
// для L построить палиндром Жегалкина

// для M написать какой-то алгоритм...

/// Проверка на монотонность
/// Алгоритм с учебника ТГУ <a href = "https://ido.tsu.ru/iop_res/bulevfunc/text/g15_5.html">click</a>
fn is_monotone_boolean_function(
    func: &util::BooleanFunction,
) -> bool {

    /*
    Определение:
    Функция монотонная,
    если для всех переменных первая половина вектора остаточной функции не превышает второй половины вектора остаточной функции
    */

    let mut len = 1usize;
    let vals = func.as_vec_bool();
    let mut ok = true;
    // рассмотреть каждый разряд
    for _ in 0..func.get_count_args() {
        // сравнить значения
        for mut j in 0..vals.len() {
            if j / len & 1 == 1 {
                j += len;
                continue;
            }
            // стоим на 0 и сравниваем c набором, где разряд принимает значение 1
            ok &= vals[j] <= vals[j + len];
            if !ok {
                return false;
            }
        }
        len *= 2;
    }

    ok
}

/// Проверка на линейность булевой функции через построение треугольника Жегалкина
fn is_linear_boolean_function(
    func: &util::BooleanFunction,
) -> bool {
    let n = func.get_func().len();
    let mut matrix = vec![ vec![0; n]; n ];

    for (i, val) in func.as_vec_bool().iter().enumerate() {
        matrix[i][0] = *val as i32;
    }
    

    // построение треугольника Жегалкина
    for i in 0..n {
        for j in 0..n {
            if j > 0 && i > 0 {
                matrix[i][j] = matrix[i-1][j-1] ^ matrix[i][j-1];
            }
            if i == j {
                break;
            }
        }
    }


    let mut ok = true;
    for (i, (args, _)) in util::BooleanFunctionIterator::new(&func).enumerate() {
        if matrix[i][i] != 1 {
            continue;
        }
        let cnt = args.iter().filter(|&&val| val == true).count();
        ok &= cnt <= 1;
    }
    
    ok
}

/// Проверка на самодвойственность булевой функции
fn is_s(
    func: &util::BooleanFunction
) -> bool {
    let a = func.as_vec_bool();
    for i in 0..a.len()/2 {
        if a[i] == a[a.len() - 1 - i] {
            return false;
        }
    }

    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monotone() {
        let func = util::BooleanFunction::from("00010111").unwrap();
        assert_eq!(is_monotone_boolean_function(&func), true);

        let func = util::BooleanFunction::from("01100111").unwrap();
        assert_eq!(is_monotone_boolean_function(&func), false);

        let func = util::BooleanFunction::from("11100011").unwrap();
        assert_eq!(is_monotone_boolean_function(&func), false);
    }

    #[test]
    fn tets_t0() {
        let func = util::BooleanFunction::from("0101").unwrap();
        assert_eq!(true, is_t0(&func));

        let func = util::BooleanFunction::from("1101").unwrap();
        assert_eq!(false, is_t0(&func));
    }

    #[test]
    fn test_t1() {
        let func = util::BooleanFunction::from("0101").unwrap();
        assert_eq!(true, is_t1(&func));
    }

    #[test]
    fn test_linear() {
        let func = util::BooleanFunction::from("10110001").unwrap();
        assert_eq!(is_linear_boolean_function(&func), false);

        let func = util::BooleanFunction::from("1000").unwrap();
        assert_eq!(is_linear_boolean_function(&func), false);


        let func = util::BooleanFunction::from("0111").unwrap();
        assert_eq!(is_linear_boolean_function(&func), false);

        let func = util::BooleanFunction::from("1001").unwrap();
        assert_eq!(is_linear_boolean_function(&func), true);
    }


    #[test]
    fn test_is_s() {
        let func = util::BooleanFunction::from("10000001").unwrap();
        assert_eq!(false, is_s(&func));

        let func = util::BooleanFunction::from("00001111").unwrap();
        assert_eq!(true, is_s(&func));

        let func = util::BooleanFunction::from("10001111").unwrap();
        assert_eq!(false, is_s(&func));

    }

}