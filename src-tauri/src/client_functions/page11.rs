use std::{collections::HashSet};

use crate::tasks::{task10and11, util};

/// Вернет T0, T1, S, M, L
#[tauri::command]
pub fn match_functions_to_precomplete_classes(functions: &str) -> Result<( bool, [bool; 5] ), String> {
    let funcs = parse_boolean_functions_set(functions)?;
    Ok(task10and11::match_functions_to_precomplete_classes(funcs))
}

fn parse_boolean_functions_set(string: &str) -> Result<HashSet<util::BooleanFunction>, String> {

    let mut set = HashSet::new();
    let mut tokens: Vec<String> = vec![];
    // оставить в строке только 0 и 1
    let mut iterator = string.chars().peekable();
    while let Some(&ch) = iterator.peek() {
        match ch {
            '0' | '1' => {
                let mut str = String::new();
                while let Some(&ch) = iterator.peek() {
                    if ch != '0' && ch != '1' {
                        break;
                    }
                    str.push(ch);
                    iterator.next();
                }
                tokens.push(str);
            }
            _ => {
                iterator.next();
            }
        }
    }
    for func in tokens {
        match util::BooleanFunction::from(&func) {
            Ok(value) => {
                set.insert(value);
            }
            Err(err) => {
                return Err(format!("Ошибка парсинга на `{}`, err: {}", func, err));
            }
        }
    }

    Ok(set)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let func = "1011, 1010, 0001, 00100101, 1010";
        let r = parse_boolean_functions_set(func);

        assert_eq!(r.is_ok(), true);

        
        let func = "101, 1010, 0001, 00100101, 1010";
        let r = parse_boolean_functions_set(func);
        assert_eq!(r.is_err(), true);
    }

    #[test]
    fn test_empty() {
        let func = "10100001, 00111100";
        println!("{:?}", match_functions_to_precomplete_classes(func));
    }

}