use crate::tasks::util;


fn parse_num_arg(n: &str) -> Result<u8, &'static str> {
    let Ok(n) = n.parse::<u8>() else {
        return Err("Ошибка парсинга! Ожидалось значение типа u8");
    };
    if n == 0 {
        return Err("Аргумент должен быть больше нуля");
    }
    Ok(n)
}

pub fn convert_num_arg(func: &util::BooleanFunction, n: &str) -> Result<u8, &'static str> {
    let n = parse_num_arg(n)?;

    if n > func.get_count_args() {
        return Err("Превышено максимальное допустимое значение");
    }
    
    Ok(func.get_count_args() - n)
}