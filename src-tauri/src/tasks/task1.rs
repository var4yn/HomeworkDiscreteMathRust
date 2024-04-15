use super::util;

/// На вход — число n, на выход — булева функция от n аргументов.

/// Теперь используется ассоциативная функция util::BooleanFunction::with_count_args().
#[deprecated]
fn get_bool_func_from_n(n: i32) -> util::BooleanFunction {
    if n >= 32 { 
       panic!("max value for n is 31");
    }

    let mut string = String::new();
    for _ in 0..(1i32<<n) {
        let val = util::get_random(2);
        string.push(std::char::from_digit(val as u32, 10).unwrap());
    }

    util::BooleanFunction::from(string).unwrap()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_valid_bool_function() {
        let r = get_bool_func_from_n(5);
        assert_eq!(r.get_count_args(), 5);
    }

}