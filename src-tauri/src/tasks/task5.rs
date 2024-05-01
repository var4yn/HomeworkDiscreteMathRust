use super::util;

/// Игра. Существенные и фиктивные переменные. 
/// Система предлагает вектор функции. Пользователь выбирает существенные и фиктивные переменные.


/// На вход вектор функции
/// Существенные и фиктивные переменные в динамическом массиве, где 0 - фиктивность, а 1 - существенность
/// [0, 1, 0] означает, что переменная x1 фиктивна, x2 существенна, x3 фиктивна
pub fn get_dummy_variable(
    func: util::BooleanFunction,
) -> Vec<bool> {
    let mut vector = Vec::new();
    vector.resize(func.get_count_args() as usize, true);

    for i in 0..func.get_count_args() {
        if func.remainde_boolean_function(i, true) == func.remainde_boolean_function(i, false) {
            vector[i as usize] = false;
        }
    }
    vector.reverse();

    vector
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bool_func = util::BooleanFunction::from("00111100").unwrap();
        let vars = get_dummy_variable(bool_func);

        for (num, i) in vars.iter().enumerate() {
            println!("{} for {}", i, num);
        }

    }

}
