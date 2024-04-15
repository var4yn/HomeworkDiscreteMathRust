use super::util;

/// На вход — два вектора (это нулевая и единичная остаточные функции по некоторому аргументу), номер аргумента, на выход — вектор функции.
fn boolean_function_from_remainde_func(
    func0: String,
    func1: String,
    num_arg: u32,
) -> Result<util::BooleanFunction, &'static str> {
    // проверка на валидность num_arg
    if 1 << num_arg >= func0.len() + func1.len() {
        return Err("Invalid value of num_arg.");
    }
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
        let res = boolean_function_from_remainde_func("1110".to_string(), "0111".to_string(), 1);
        assert_eq!(util::BooleanFunction::from("11011011").unwrap(), res.unwrap());

        let res = boolean_function_from_remainde_func("1101".to_string(), "1011".to_string(), 2);
        assert_eq!(util::BooleanFunction::from("11011011").unwrap(), res.unwrap());

        let res = boolean_function_from_remainde_func("0000".to_string(), "1100".to_string(), 0);
        assert_eq!(util::BooleanFunction::from("01010000").unwrap(), res.unwrap());
    }

    #[test]
    fn test_error() {
        let b = boolean_function_from_remainde_func("1111".to_string(), "0000".to_string(), 0);
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