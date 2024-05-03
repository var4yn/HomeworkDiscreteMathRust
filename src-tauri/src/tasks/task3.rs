use super::util;

/// На вход — два вектора (это нулевая и единичная остаточные функции по некоторому аргументу), номер аргумента, на выход — вектор функции.
pub fn boolean_function_from_remainde_func(
    func0: &str,
    func1: &str,
    num_arg: u8,
) -> Result<util::BooleanFunction, &'static str> {
    // проверка на валидность num_arg
    if 1 << num_arg >= func0.len() + func1.len() {
        return Err("Invalid value of num_arg.");
    }
    // проверка на равенство остаточных функций
    if func0.len() != func1.len() {
        return Err("Expected equal lengths.");
    }

    let mut string = String::new();

    let mut a = [func0.chars(), func1.chars()];

    let len = 1usize<<num_arg;

    for i in 0..2*func0.len() {
        string.push( a[ i/len & 1 ].next().unwrap() );
    }

    util::BooleanFunction::from(string)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_func() {
        let res = boolean_function_from_remainde_func("1110", "0111", 1);
        assert_eq!(util::BooleanFunction::from("11011011").unwrap(), res.unwrap());

        let res = boolean_function_from_remainde_func("1101", "1011", 2);
        assert_eq!(util::BooleanFunction::from("11011011").unwrap(), res.unwrap());

        let res = boolean_function_from_remainde_func("0000", "1100", 0);
        assert_eq!(util::BooleanFunction::from("01010000").unwrap(), res.unwrap());
    }

    #[test]
    fn test_error() {
        let b = boolean_function_from_remainde_func("1111", "0000", 0);
        match b {
            Ok(_b) => {
                //
            }
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }
}