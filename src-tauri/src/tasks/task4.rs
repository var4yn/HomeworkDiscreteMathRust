use std::cmp::min;

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

/// Возвращает вектор функции и его индекс
pub fn get_random_bynary_boolean_func() -> (&'static str, &'static str) {
    let i = util::get_random(BINARY_BOOLEAN_FUNCS.len() as u32) as usize;
    (BINARY_BOOLEAN_FUNCS[i].0, BINARY_BOOLEAN_FUNCS[i].1)
}

pub fn get_name_funcs() -> Vec<&'static str> {
    let mut vc = Vec::new();
    for (_, name) in BINARY_BOOLEAN_FUNCS {
        vc.push(name);
    }

    vc
}