use super::util;

/// На вход — вектор функции, 0 или 1, номер аргумента, на выход — соответствующая остаточная
/// num_arg - номер аргумента. Индексация с 1 и идёт слева направо
fn remainde_boolean_function(
    boolean_function: util::BooleanFunction,
    value: bool,
    num_arg: u8,
) -> Result<String, &'static str> {
    let num_arg = boolean_function.get_count_args() - 1 - num_arg;
    boolean_function.remainde_boolean_function(num_arg, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remainder_function() {
        let func = util::BooleanFunction::from("11011011").unwrap();
        let num = 1;
        let c = true;
        assert_eq!("0111", remainde_boolean_function(func, c, num).unwrap());


        let func = util::BooleanFunction::from("1110011001101110").unwrap();
        let num = 0;
        let c = true;
        assert_eq!("01101110", remainde_boolean_function(func, c, num).unwrap());
    }

    #[test]
    fn test_func() {
        let r = remainde_boolean_function(util::BooleanFunction::from("00").unwrap(), false, 0);
        match r {
            Ok(val) => {
                assert_eq!(val, "0");
            }
            Err(msg) => {
                println!("{}", msg);
            }
        }
    }

    #[test]
    fn test_ffunc() {
        assert_eq!(remainde_boolean_function(util::BooleanFunction::from("01010001").unwrap(), false, 0).unwrap(), "0101".to_string());
        assert_eq!(remainde_boolean_function(util::BooleanFunction::from("01010001").unwrap(), true, 0).unwrap(), "0001".to_string());
    }

    #[test]
    fn ts_struct() {
        let r = util::BooleanFunction::from("01010001").unwrap();
        assert_eq!(r.remainde_boolean_function(2, true).unwrap(), "0001".to_string());
    }

}