use super::util;
///! Игра. Узнать имя функции от 2-х аргументов.
///! Система предлагает вектор функции, пользователь выбирает «имя» (одно из 16).

/// 16 бинарных булевых функций
static BYNARY_BOOLEAN_FUNCS: [(&'static str, &'static str); 16] = [
    ("0000", "0"),
    ("0001", "x ∧ y"),
    ("0010", "x ↛ y"),
    ("0011", "x"),
    ("0100", "x ↚ y"),
    ("0101", "y"),
    ("0110", "x⊕y"),
    ("0111", "x ∨ y"),
    ("1000", "x ↓ y"),
    ("1001", "x = y"),
    ("1010", "¬y"),
    ("1011", "x ← y"),
    ("1100", "¬x"),
    ("1101", "x → y"),
    ("1110", "x ▽ y"),
    ("1111", "1"),
];

/// Возвращает вектор функции
fn get_random_bynary_boolean_func() -> (&'static str, i32) {
    let i = util::get_random(16);
    (BYNARY_BOOLEAN_FUNCS[i as usize].0, i)
}

/// Сравнивает по вектору функции совпадение по индексу в массиве
fn check_equal_binary_boolean_func(func: String, index: i32) -> bool {
    BYNARY_BOOLEAN_FUNCS[index as usize].0 == func
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let r = get_random_bynary_boolean_func();
        check_equal_binary_boolean_func(r.0.to_string(), r.1);
    }

}