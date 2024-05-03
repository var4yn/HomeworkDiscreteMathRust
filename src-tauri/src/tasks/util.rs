use rand::Rng;

/// Хранит валидное значение булевой функции
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanFunction {
    func: String,
    count_arguments: u8,
}

impl BooleanFunction {
    const MAX_COUNT_ARGS: u8 = 10;

    fn check_valid_value(str: &String) -> bool {
        let mut ok = true;
        for i in str.chars() {
            ok &= i == '0' || i == '1';
        }
        ok
    }
    fn check_valid_size(len: usize) -> Result<(bool, u8), &'static str> {
        let mut r = 1usize;
        let mut count = 1;

        while r << 1 < len {
            if count >= Self::MAX_COUNT_ARGS {
                return Err("Maximum length of Boolean Function exceeded.");
            }
            r <<= 1;
            count += 1;
        }

        return Ok((1usize << count == len, count));
    }
    
    pub fn from(str: impl Into<String>) -> Result<Self, &'static str> {
        let str = str.into();
        if !Self::check_valid_value(&str) {
            return Err("Function must be consist by 0 or 1.")
        }
        match Self::check_valid_size(str.len()) {
            Ok((true, len)) => Ok(BooleanFunction{func: str, count_arguments: len}),
            Ok((false, _)) => Err("Function length must be a power of two."),
            Err(e) => Err(e)
        }
    }

    /// Возвращает остаточную булевую функцию по номеру аргумента и его значению
    /// Индексация с 0 и идёт справо налево
    pub fn remainde_boolean_function(&self, num_arg: u8, value: bool) -> Result<String, &'static str> {
        // возвращает string :<
        // прикол в том, что булевая функция из одного аргумента вернет вектор функции длиной 1
        // это является невалидным значением для структуры util::BooleanFunction

        if num_arg >= self.count_arguments {
            return Err("Argument number greater than maximum.");
        }        

        let len = 1usize << num_arg;
        let mut string = String::new();

        for (i, ch) in self.func.chars().enumerate() {
            if (i / len & 1) as u8 == value as u8 {
                string.push(ch);
            }

        }
        
        Ok(string)
    }

    /// Возвращает случайную булевую функцию из n аргументов
    pub fn with_count_args(n: u8) -> Self {
        let n = std::cmp::max(n, 1);
        let n = std::cmp::min(n, Self::MAX_COUNT_ARGS);

        let mut string = String::new();
        for _ in 0..(1i32<<n) {
            let val = get_random(2);
            string.push(std::char::from_digit(val, 10).unwrap());
        }

        BooleanFunction::from(string).unwrap()
    }


    pub fn get_count_args(&self) -> u8 {
        self.count_arguments
    }

    pub fn get_func(&self) -> &String {
        &self.func
    }

    pub fn as_vec_bool(&self) -> Vec<bool> {
        let mut vector = Vec::new();
        for i in self.func.chars() {
            vector.push(i == '1');
        }

        vector
    }

    pub fn have_dnf(&self) -> bool {
        self.func.contains('1')
    }

    pub fn have_knf(&self) -> bool {
        self.func.contains('0')
    }

}

pub fn get_random(n: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..n);

    random_number
}

impl<'a> IntoIterator for &'a BooleanFunction {
    type Item = (Vec<bool>, bool);
    type IntoIter = BooleanFunctionIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BooleanFunctionIterator::new(&self)
    }

}

/// Итератор по булевой функции
/// Возвращает вектор текущих значений аргументов функции и текущее значение функции
pub struct BooleanFunctionIterator<'a> {
    current: i32,
    cnt: usize,
    end: bool,
    func_it: std::str::Chars<'a>,
}

impl<'a> BooleanFunctionIterator<'a> {
    pub fn new(bool_func: &'a BooleanFunction) -> BooleanFunctionIterator<'a> {
        BooleanFunctionIterator{ current: 0, cnt: bool_func.count_arguments as usize, func_it: bool_func.func.chars(), end : false }
    }
}

impl<'a> Iterator for BooleanFunctionIterator<'a> {
    type Item = (Vec<bool>, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end {
            return None;
        }
        let mut ok = true;
        let mut vc = vec![ false; self.cnt ];

        let mut len = 1;
        for j in (0..self.cnt).rev() {
            let current_value = self.current / len & 1;
            len *= 2;
            vc[j] = current_value == 1;
            ok &= vc[j];
        }
        self.end = ok;
        self.current += 1;
        Some((vc, self.func_it.next() == Some('1')))
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_func() {
        let mut rng = rand::thread_rng();
        

        let mut ok = true;
        for _ in 1..50 {
            let random_number = rng.gen_range(0..5);
            ok &= random_number >= 0 && random_number <= 5;
        }
        assert_eq!(ok, true);
    }

    #[test]
    #[should_panic]
    fn check_struct_panic_value() {
        BooleanFunction::from("abc").unwrap();
    }

    #[test]
    #[should_panic]
    fn check_struct_panic_length() {
        BooleanFunction::from("1010101010").unwrap();
    }

    #[test]
    fn check_create_struct() {
        let r = BooleanFunction::from("10").unwrap();
        println!("{}", r.count_arguments);
    }

    #[test]
    fn test_iterator() {
        let func = "10100011";
        let ff = BooleanFunction::from(func).unwrap();

        let it =  BooleanFunctionIterator::new(&ff);
        
        for i in it {
            println!("{:?}", i);
        }

        for i in &ff {
            println!("{:?}", i);
        }

    }

    #[test]
    fn test_func_0000_has_dnf() {
        let r = BooleanFunction::from("0000").unwrap();
        assert_eq!(r.have_dnf(), false);
    }

    #[test]
    fn test_func_0000_has_knf() {
        let r = BooleanFunction::from("1111").unwrap();
        assert_eq!(r.have_knf(), false);
    }

}