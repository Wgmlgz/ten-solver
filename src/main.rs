use std::io::stdin;

use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, EnumIter)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn main() {
    let target = 10.;
    let n = 4;
    let mut v: Vec<i32>;

    loop {
        let mut s = String::new();

        println!("Enter string of 4 numbers:");
        stdin().read_line(&mut s).unwrap();

        s = s.trim().to_string();

        if !Regex::new("^[0-9]{4}$").unwrap().is_match(&s) {
            println!("invalid string (");
            continue;
        }

        v = s
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32)
            .collect();

        let digits_iter = v.iter().permutations(n).unique();
        let ops_iter = std::iter::repeat(Op::iter())
            .take(n - 1)
            .multi_cartesian_product();

        for digits in digits_iter {
            for ops in ops_iter.clone() {
                let mut stack = digits.iter().map(|x| (**x) as f64).collect_vec();

                for op in ops.clone() {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    let res = match op {
                        Op::Add => a + b,
                        Op::Sub => a - b,
                        Op::Mul => a * b,
                        Op::Div => a / b,
                    };
                    stack.push(res);
                }
                let res = stack.pop().unwrap();

                if res == target {
                    println!("{:?}, {:?}, {target}", digits, ops);
                }
            }
        }
    }
}
