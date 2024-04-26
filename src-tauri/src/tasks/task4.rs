use super::util;
/// Игра. Узнать имя функции от 2-х аргументов.
/// Система предлагает вектор функции, пользователь выбирает «имя» (одно из 16).

/// 16 бинарных булевых функций
static BINARY_BOOLEAN_FUNCS: [(&'static str, &'static str); 16] = [
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
pub fn get_random_bynary_boolean_func() -> (&'static str, usize) {
    let i = util::get_random(BINARY_BOOLEAN_FUNCS.len() as u32) as usize;
    (BINARY_BOOLEAN_FUNCS[i as usize].0, i)
}

/// Сравнивает по вектору функции совпадение по индексу в массиве
pub fn check_equal_binary_boolean_func(func: &str, index: usize) -> bool {
    BINARY_BOOLEAN_FUNCS[index].0 == func
}

pub fn get_name_funcs() -> Vec<&'static str> {
    let mut vc = Vec::new();
    for (_, name) in BINARY_BOOLEAN_FUNCS {
        vc.push(name);
    }

    vc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let r = get_random_bynary_boolean_func();
        check_equal_binary_boolean_func(r.0, r.1);
    }

}