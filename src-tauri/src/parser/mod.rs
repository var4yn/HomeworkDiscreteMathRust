/// Парсер логических выражений

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Token {
    Value(bool),
    Variable(String),
    Operator(char),
}

#[derive(Debug)]
pub enum Expression {
    Value(bool),
    Variable(String),
    UnaryOp(char, Box<Expression>),
    BinaryOp(char, Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn evaluate(&self, vars: &HashMap<String, bool>) -> Result<bool, String> {
        match self {
            Expression::Value(value) => Ok(*value),
            Expression::Variable(var) => {
                if !vars.contains_key(var) {
                    Err(format!("Not expected var : {}", var))
                } else {
                    Ok(vars.get(var).unwrap().clone())
                }
                
            },
            Expression::UnaryOp(op, expr) => {
                let val = expr.evaluate(vars);
                if let Err(e) = val {
                    return Err(e);
                }
                match op {
                    '-' => Ok(!val.unwrap()),
                    _ => Err("Invalid unary operator!".to_string()),
                }
            }
            Expression::BinaryOp(op, left, right) => {
                let l_val = left.evaluate(vars);
                if let Err(e) = l_val {
                    return Err(e);
                }
                let r_val = right.evaluate(vars);
                if let Err(e) = r_val {
                    return Err(e);
                }
                match op {
                    '&' => Ok(l_val.unwrap() && r_val.unwrap()),
                    'v' => Ok(l_val.unwrap() || r_val.unwrap()),
                    _ => Err("Invalid binary operator!".to_string()),
                }
            }
        }
    }

    #[allow(dead_code)]
    fn get_string(&self) -> String {
        match self {
            Expression::Value(value) => format!("{}", value),
            Expression::Variable(var) => var.clone(),
            Expression::UnaryOp(op, expr) => {
                let val = expr.get_string();
                format!("{}{}", op, val)
            }
            Expression::BinaryOp(op, left, right) => {
                let l_val = left.get_string();
                let r_val = right.get_string();
                format!("({}{}{})", l_val, op, r_val)
            }
        }
    }
}

pub mod parse {

    use super::*;

    fn tokenize(expression: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut iter = expression.chars().peekable();
    
        while let Some(&c) = iter.peek() {
            match c {
                '&' | 'v' | '(' | ')' | '-'  => {
                    tokens.push(Token::Operator(c));
                    iter.next();
                }
                'x' => {
                    let mut var_str = String::new();
                    while let Some(&c) = iter.peek() {
                        if c == 'v' {
                            break;
                        }
                        if c != 'x' && !c.is_digit(10) {
                            break;
                        }
                        var_str.push(c);
                        iter.next();
                    }
                    tokens.push(Token::Variable(var_str));
                }
                '0' | '1' => {
                    tokens.push(Token::Value(c == '1'));
                    iter.next();
                }
                _ => {
                    iter.next();
                }
            }
        }
    
        tokens
    }
    
    
    fn parse_expression(tokens: &mut Vec<Token>, precedence: u8) -> Result<Expression, String> {
    
        let lhs = parse_term(tokens);
        if let Err(e) = lhs {
            return Err(e);
        }
        let mut lhs = lhs?;
    
        while let Some(&Token::Operator(op)) = tokens.last() {
            let op_precedence = match op {
                '&' => 2,
                'v' => 1,
                _ => 0,
            };
    
            if precedence >= op_precedence {
                break;
            }
    
            tokens.pop();
            let rhs = parse_expression(tokens, op_precedence);
            if let Err(e) = rhs {
                return Err(e);
            }
            
            lhs = Expression::BinaryOp(op, Box::new(lhs), Box::new(rhs?));
        }
    
        
        Ok(lhs)
    
    }
    
    fn parse_term(tokens: &mut Vec<Token>) -> Result<Expression, String> {
        match tokens.pop().unwrap() {
            Token::Value(val) => Ok(Expression::Value(val)),
            Token::Variable(var) => Ok(Expression::Variable(var)),
            Token::Operator(op) => {
                match op {
                    '(' => {
                        let expr = parse_expression(tokens, 0);
                        if let Err(e) = expr {
                            return Err(e);
                        }
                        if let Some(Token::Operator(')')) = tokens.pop() {
                            Ok(expr?)
                        } else {
                            Err(format!("Expected ')' but found: '{}'", op))
                        }
                    }
                    '-' => {
                        let expr = parse_term(tokens);
                        if let Err(e) = expr {
                            return Err(e);
                        }
                        Ok(Expression::UnaryOp('-', Box::new(expr?)))
                    }
                    _ => Err(format!("Invalid term: {}", op)),
                }
            }
        }
    }

    pub fn get_ast_tree(expression : &str) -> Result<Expression, String> {
        let mut tokens = tokenize(expression);
        tokens.reverse();
        parse_expression(&mut tokens, 0)
    }
    

}

pub mod validate {
    use super::*;

    /*

    /// Обход графа в глубину
    /// Проверяет, что после определенной операции нет другой
    /// Проще говоря, проверка на конъюнктор/дизъюнктор
    /// в ( x1 & x2 & x3 ) - валидный конъюнктор, но (x1 & x2 & x3 v x4) - невалидный
    fn dfs(expression: &Expression, ch: char, mut flag: bool) -> bool {
        match expression {
            Expression::BinaryOp(op, left, right) => {
                if *op != ch && flag {
                    return false;
                }
                flag = *op == ch;
                dfs(left, ch, flag) && dfs(right, ch, flag)
            }
            _ => true,
        }
    }
    
    */

    /// комбинация функции dfs и dfs_t
    fn dfs_n(expression: &Expression, ch: char, mut flag: bool) -> (bool, HashSet<String>) {
        let mut set : HashSet<String> = HashSet::new();
    
        match expression {
            Expression::BinaryOp(op, left, right) => {
                if *op != ch && flag {
                    return (false, set)
                }
                flag = *op == ch;
                let (ok_l, lv) = dfs_n(left, ch, flag);
                let (ok_r, rv) = dfs_n(right, ch, flag);
                let fl = ok_l && ok_r;
                if *op == ch {
                    // не должно быть повторений
                    let mut ok = true && fl;
                    set = lv;
                    for i in rv {
                        if set.contains(&i) {
                            ok &= false;
                        } else {
                            set.insert(i);
                        }
                    }
                    (ok, set)
                } else {
                    // повторения возможны
                    for i in rv {
                        set.insert(i);
                    }
                    (fl, set)
                }
            }
            Expression::UnaryOp(_, expr) => {
                dfs_n(expr, ch, flag)
            }
            Expression::Variable(var) => {
                set.insert(var.to_string());
                (true, set)
            }
            _ => (true, set),
        }
    
    
    }
    
    /*
    #[deprecated]
    // проверяет на повторение переменных в выражении
    // x1&x2&x2 -> false т.к. повторилось x2 
    // (x1 v x2 v -x2 ) -> false т.к. повторилось x2
    fn dfs_t(expression: &Expression, ch: char) -> (bool, HashSet<String>) {
        let mut set : HashSet<String> = HashSet::new();
    
        match expression {
            Expression::BinaryOp(op, left, right) => {
                let (ok_l, lv) = dfs_t(left, ch);
                let (ok_r, rv) = dfs_t(right, ch);
                let fl = ok_l && ok_r;
                if *op == ch {
                    // не должно быть повторений
                    let mut ok = true && fl;
                    set = lv;
                    for i in rv {
                        if set.contains(&i) {
                            ok &= false;
                        } else {
                            set.insert(i);
                        }
                    }
                    (ok, set)
                } else {
                    // повторения возможны
                    for i in rv {
                        set.insert(i);
                    }
                    (fl, set)
                }
            }
            Expression::UnaryOp(_, expr) => {
                dfs_t(expr, ch)
            }
            Expression::Variable(var) => {
                set.insert(var.to_string());
                (true, set)
            }
            _ => (true, set),
        }
    }
    */
    
    pub fn is_dnf(expression: &Expression) -> bool {
        dfs_n(expression, '&', false).0
    }

    pub fn is_knf(expression: &Expression) -> bool {
        dfs_n(expression, 'v', false).0
    }


}








#[cfg(test)]
mod tests {
    use super::*;

    fn get_map_from_vals(vals: Vec<bool>) -> HashMap<String, bool> {
        let mut mpp = HashMap::new();
        for (i, val) in vals.iter().enumerate() {
            mpp.insert(format!("x{}", i + 1), val.clone());
        }
    
        mpp
    }
    

    #[test]
    fn test_parse_dnf() {
        let expression = " (x1 & x2) v (x1 & x2) ";

        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " (x1) v (x2) ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " x1 v x2 ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " x3&x2&-x1 v x1&x2 v x3 ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " x1&x2 ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), true);


        let expression = " x2&x3&-x1 v x2&-x2 ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), false);
        let expression = " x1&-x1 ";
        assert_eq!(validate::is_dnf(&parse::get_ast_tree(expression).unwrap()), false);
    }


    #[test]
    fn test_calculate_dnf() {
        let expression = " (x1 & x2) v (x1 & x2) ";

        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, true])), Ok(true));
        let expression = " (x1) v (x2) ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, false])), Ok(true));
        let expression = " x1 v x2 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![false, true])), Ok(true));
        let expression = " x3&x2&-x1 v x1&x2 v x3 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![false, false, true])), Ok(true));
        let expression = " x1&x2 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, true])), Ok(true));
        
    }



    #[test]
    fn test_parse_knf() {
        // (x1 v x2 v ... v x10) & ( y v z )

        let expression = " (x1 v x2) & (x1 v x2) ";

        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " (x1) & (x2) ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " x1 & x2 ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " (x3vx2v-x1) & (x1vx2) & x3 ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), true);
        let expression = " x1 v x2 ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), true);

        let expression = " x2&x3& -x1 v x2 & -x2 ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), false);
        let expression = " x1 v -x1 ";
        assert_eq!(validate::is_knf(&parse::get_ast_tree(expression).unwrap()), false);
    }

    #[test]
    fn test_calculate_knf() {
        let expression = " (x1 v x2) & (x1 v x2) ";

        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, false])), Ok(true));
        let expression = " (x1) & (x2) ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, true])), Ok(true));
        let expression = " x1 & x2 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![false, true])), Ok(false));
        let expression = " (x3vx2v-x1) & (x1vx2) & x3 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![false, true, true])), Ok(true));
        let expression = " x1 v x2 ";
        assert_eq!(parse::get_ast_tree(expression).unwrap().evaluate(&get_map_from_vals(vec![true, false])), Ok(true));
    }


    #[test]
    fn test_safe_ast() {
        let expr = " x1 v 1&0 ";

        let r = parse::get_ast_tree(expr);
        let result = r.unwrap().evaluate(&get_map_from_vals(vec![false])).unwrap();
        println!("result = {}", result);


    }

}