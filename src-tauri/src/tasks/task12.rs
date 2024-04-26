use std::{cmp::{max, min}, collections::HashMap};

use super::util;

/// Построение ДНФ с помощью метода Куайна — Мак-Класки

pub fn get_expression_dnf_with_qms(
    func: util::BooleanFunction,
) -> String {
    // объединение по кол-ву единиц в группы
    let (groups, mx_cnt) = create_groups(&func);
    
    // создание первых склеек
    let union_groups = build_unions(groups, func.get_count_args() as usize, mx_cnt);

    let mut result = make_qmc(union_groups);
    result.sort();
    result.dedup(); // удаляем дубликаты
    
    get_expression(result)
}

fn get_expression(vc: Vec<Vec<char>>) -> String {
    let mut expr = String::new();

    for (j, union) in vc.iter().enumerate() {
        expr.push_str(&get_conjunctor(union));
        if j + 1 != vc.len() {
            expr.push_str(" v ");
        }
    }

    expr
}

/// Строит конъюнктор по вектору символов
fn get_conjunctor(item: &Vec<char>) -> String {
    let mut conjunctor = String::new();
    for (i, &ch) in item.iter().enumerate() {
        if ch == '.' {
            continue;
        }
        if !conjunctor.is_empty() {
            conjunctor.push('&');
        }

        if ch == '1' {
            conjunctor.push_str(&format!("x{}", i + 1));
        } else {
            conjunctor.push_str(&format!("-x{}", i + 1));
        }
    }

    conjunctor
}

/// Делает склейки с точками
fn make_qmc(union_groups: Vec<Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let mut union_groups = union_groups;
    let mut un_groups = vec![ vec![] ];

    let mut flag = true;
    while flag {
        flag = false;
        un_groups = vec![ vec![]; union_groups.len() ];
        for (num, group) in union_groups.iter().enumerate() {
            let mut map = vec![ false; group.len() ];
            for i in 0..group.len() {
                if map[i] {
                    continue;
                }
                let mut ok = false;
                for j in i+1..group.len() {
                    if let (true, mut pos) = is_similar(&group[i], &group[j]) {
                        ok = true;
                        flag = true;
                        map[j] = true;
                        let mut new_union = group[i].to_owned();
                        new_union[pos] = '.';
                        //println!("{:?}", &new_union);
                        pos = min(pos, num);
                        un_groups[pos].push(new_union);                        
                    }
                }
                if !ok {
                    un_groups[num].push(group[i].to_owned());
                }
            }
        }
        if flag {
            std::mem::swap(&mut union_groups, &mut un_groups);
        }
    }
    

    un_groups
        .into_iter()
        .flatten()
        .collect()
}

/// Создает группы по количеству единиц в двоичных наборах
fn create_groups(
    func: &util::BooleanFunction
) -> (HashMap<usize, Vec<Vec<char>>>, usize) {
    let mut groups: HashMap<usize, Vec< Vec<char> >> = HashMap::new();
    let mut mx_cnt = 0;
    // строим группы по количеству единиц в двоичных наборах
    for (args, val) in func {
        if val == false {
            continue;
        }
        let cnt = args.iter().filter(|&&el| el == true).count();
        mx_cnt = max(mx_cnt, cnt);
        groups.entry(cnt).or_insert(vec![]).push(to_chars_from_vc_bool(args));
    }

    (groups, mx_cnt)
}

/// Делает первую склейку
fn build_unions(
    groups: HashMap<usize, Vec<Vec<char>>>,
    count_args: usize,
    mx_cnt: usize,
) -> Vec<Vec<Vec<char>>> {
    let mut union_groups = vec![ vec![]; count_args as usize];

    // затем делаем склейки группы i с группой i+1
    for i in 0..mx_cnt {
        // условие что есть i и есть i + 1
        if let (Some(cur), Some(next)) = ( groups.get(&i), groups.get(&(i+1)) ) {
            // сопоставление наборов из группы i с группой i+1
            // пробег по наборам текущей группы
            for first in cur {
                // сопоставить с наборами другой группы
                for second in next {
                    if let (true, pos) = is_similar(&first, &second) {
                        let mut union = first.to_owned();
                        union[pos] = '.';
                        // склейку нужно закинуть в новую группу, которая формируется, исходя из позиции точки
                        union_groups[pos].push(union);
                    }
                }
            }
        }
    }

    union_groups
}

/// Переводит из векторов bool в вектор символов
fn to_chars_from_vc_bool(
    value: Vec<bool>,
) -> Vec<char> {
    let mut str = Vec::new();
    for i in value {
        if i == true {
            str.push('1');
        } else {
            str.push('0');
        }
    }
    str
}

/// Проверка на отличие в одном символе
fn is_similar(
    first: &Vec<char>,
    second: &Vec<char>,
) -> (bool, usize) {
    if first.len() != second.len() {
        panic!("Length must be equal.");
    }

    let (cnt, last_pos) = first.iter()
        .zip(second.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(pos, _)| pos)
        .fold((0, 0), | (count, _), pos| (count + 1, pos));

    (cnt == 1, last_pos)
}


#[cfg(test)]
mod tests {

    use crate::tasks::task6and7::check_dnf;

    use super::*;

    #[test]
    fn test_func() {
        let func = util::BooleanFunction::from("10110011").unwrap();
        let r = get_expression_dnf_with_qms(func.clone());

        assert_eq!(true, check_dnf(func, &r).unwrap());

    }

    #[test]
    fn test_valid() {
        let func = util::BooleanFunction::from("0001110101011100").unwrap();
        let r = get_expression_dnf_with_qms(func.clone());
        assert_eq!(true, check_dnf(func, &r).unwrap());
    }



    #[test]
    fn test_splice() {
        let func = util::BooleanFunction::from("0001110101011100").unwrap();
        let (groups, mx_cnt) = create_groups(&func);
        let union_groups = build_unions(groups, func.get_count_args() as usize, mx_cnt);
        let res = vec![
            vec![vec!['.', '1', '0', '0'],
            vec!['.', '0', '1', '1'],
            vec!['.', '1', '0', '1'],],
            vec![vec!['0', '.', '1', '1'],
            vec!['1', '.', '0', '1'],],
            vec![vec!['0', '1', '.', '1'],
            vec!['1', '0', '.', '1'],],
            vec![vec!['0', '1', '0', '.'],
            vec!['1', '1', '0', '.'],],
        ];
        let mut ok = true;
        for i in 0..res.len() {
            for j in 0..res[i].len() {
                for x in 0..res[i][j].len() {
                    ok &= res[i][j][x] == union_groups[i][j][x];
                }

            }
        }

        assert_eq!(ok, true);
    }

}