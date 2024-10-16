use std::{fmt::Display, str::FromStr};

pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
    Ok("Day 1 completed".to_string())
}

fn part_1(input: &str) -> Result<String, String> {
    // println!("{}", input);
    let n_lines = input.lines().count();
    let mut calibration_codes = vec![-1; n_lines];
    for (i, line) in input.lines().enumerate() {
        // as an improvement, it can be parallelized
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<_>>();
        let mut number = digits.first().unwrap().to_string();
        number.push_str(&digits.last().unwrap().to_string());
        let number = number.parse::<i32>().unwrap();
        //println!("{}",number);
        calibration_codes[i] = number;
    }
    //calibration_codes.iter().for_each(|n| print!("{} ", n));
    let result: i32 = calibration_codes.iter().sum();
    Ok(result.to_string())
}

#[derive(Clone, PartialEq)]
struct Node<T: Clone> {
    value: T,
    children: Vec<Node<T>>,
}
impl<T: Clone> Node<T> {
    fn new(el: T) -> Node<T> {
        Node::<T> {
            value: el,
            children: Vec::<Node<T>>::new(),
        }
    }

    fn add_child(mut self, child: NodeOrT<T>) -> Self {
        let ch = match child {
            NodeOrT::N(n) => n,
            NodeOrT::T(el) => Node::new(el)
        };
        self.children.push(ch);
        self
    }

    fn add_children(mut self, new_child: Vec<NodeOrT<T>>) -> Self {
        for ch in new_child.iter() {
            self.children.push(match ch { NodeOrT::N(c) => c.clone(), NodeOrT::T(val) => Node::new(val.clone()) });
        }
        self
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}
enum NodeOrT<T: Clone> {
    N(Node<T>), T(T)
}

struct Tree<T: Clone> {
    root: Option<Node<T>>,
}

impl<T: Clone> Tree<T> {
    fn new(node: Option<Node<T>>) -> Tree<T> {
        Tree::<T> { root: node }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Numbers {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseNumberEnumError;

impl FromStr for Numbers {
    type Err = ParseNumberEnumError;
    fn from_str(input: &str) -> Result<Numbers, Self::Err> {
        match input.to_lowercase().as_str() {
            "0" | "zero" => Ok(Numbers::Zero),
            "1" | "one" => Ok(Numbers::One),
            "2" | "two" => Ok(Numbers::Two),
            "3" | "three" => Ok(Numbers::Three),
            "4" | "four" => Ok(Numbers::Four),
            "5" | "five" => Ok(Numbers::Five),
            "6" | "six" => Ok(Numbers::Six),
            "7" | "seven" => Ok(Numbers::Seven),
            "8" | "eight" => Ok(Numbers::Eight),
            "9" | "nine" => Ok(Numbers::Nine),
            _ => Err(ParseNumberEnumError)
        }
    }
}

impl Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let num = match self {
            Numbers::Zero => "0",
            Numbers::One => "1",
            Numbers::Two => "2",
            Numbers::Three => "3",
            Numbers::Four => "4",
            Numbers::Five => "5",
            Numbers::Six => "6",
            Numbers::Seven => "7",
            Numbers::Eight => "8",
            Numbers::Nine => "9",
        };
        write!(f, "{}", num)
    }
}

fn find_first_number(line: String, tree: &Tree<String>, reversed: bool) -> Result<Numbers, ParseNumberEnumError> {
    let root = tree.root.as_ref().expect("Pass a tree for finding the elements");
    let mut cur = root;
    let mut prev: Vec<_> = vec![];
    let line_chars = line.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < line_chars.len() {
        let char = line_chars[i];
        if !cur.has_children() {
             // println!("found number");
            break;
        }
        let children = &cur.children;
        let child_opt = children.iter().find(|&child| child.value == char.to_string());
        cur = match child_opt {
            Some(child) => {
                  // println!("found, {line}, i: {i}, char: {char}, >0: {}, <len-1: {}, cur_neq_root: {}, tot: {}, len: {}", i > 0, i < line_chars.len()-1, cur != root, i > 0 /*&& i < line_chars.len()-1 */&& cur != root, line_chars.len());
                prev.push(char.to_string());
                child
            },
            None => {
                let prev_len = prev.len();
                prev = vec![];
                // println!("not found, {line}, i: {i}, char: {char}, >0: {}, <len-1: {}, cur_neq_root: {}, tot: {}, len: {}", i > 0, i < line_chars.len()-1, cur != root, i > 0 /*&& i < line_chars.len()-1 */&& cur != root, line_chars.len());
                i -= if i > 0 /*&& i < line_chars.len() - 1*/ && cur != root {prev_len} else {0};
                root
            }
        };
        i += 1;
    }
    let number_chars = if reversed { prev.into_iter().rev().collect::<String>() } else { prev.into_iter().collect::<String>() };
    Numbers::from_str(&number_chars)
}

fn part_2(input: String) -> Result<String, String> {
    let n_lines = input.lines().count();
    let mut calibration_codes = vec![-1; n_lines];
    let beginning_tree = new_trie();
    let ending_tree = new_trie_reversed();
    for (i, line) in input.lines().enumerate() {
        let beginning_number = find_first_number(line.to_owned(), &beginning_tree, false).expect("Beginning number not found").to_string();
        let ending_number = find_first_number(line.chars().rev().collect::<String>(), &ending_tree, true).expect("Ending number not found").to_string();
        // let nums = format!("num1: {}; num2: {}", beginning_number, ending_number);
        let number = beginning_number + ending_number.as_str();
        let number = number.parse::<i32>().unwrap();
        calibration_codes[i] = number;
        // println!("line: {}; {}; code: {}", line, nums, number);
    }
    //calibration_codes.iter().for_each(|n| print!("{} ", n));
    // println!("{}", calibration_codes.iter().any(|&el| el == -1));
    let result: i32 = calibration_codes.iter().sum();
    Ok(result.to_string())
}

fn new_trie() -> Tree<String> {
    use NodeOrT::*;
    let root = Node::new("".to_string());
    let z= N(Node::new("z".to_string()).add_child(N(Node::new("e".to_string()).add_child(N(Node::new("r".to_string()).add_child(T("o".to_string())))))));
    let o = N(Node::new("o".to_string()).add_child(N(Node::new("n".to_string()).add_child(T("e".to_string())))));
    let t = N(Node::new("t".to_string()).add_children(vec![
        N(Node::new("w".to_string()).add_child(T("o".to_string()))),
        N(Node::new("h".to_string()).add_child(N(Node::new("r".to_string()).add_child(N(Node::new("e".to_string()).add_child(T("e".to_string())))))))
    ]));
    let f = N(Node::new("f".to_string()).add_children(vec![
        N(Node::new("o".to_string()).add_child(N(Node::new("u".to_string()).add_child(T("r".to_string()))))),
        N(Node::new("i".to_string()).add_child(N(Node::new("v".to_string()).add_child(T("e".to_string())))))
    ]));
    let s = N(Node::new("s".to_string()).add_children(vec![
        N(Node::new("i".to_string()).add_child(T("x".to_string()))),
        N(Node::new("e".to_string()).add_child(N(Node::new("v".to_string()).add_child(N(Node::new("e".to_string()).add_child(T("n".to_string())))))))
    ]));
    let e = N(Node::new("e".to_string()).add_child(N(Node::new("i".to_string()).add_child(N(Node::new("g".to_string()).add_child(N(Node::new("h".to_string()).add_child(T("t".to_string())))))))));
    let n = N(Node::new("n".to_string()).add_child(N(Node::new("i".to_string()).add_child(N(Node::new("n".to_string()).add_child(T("e".to_string())))))));
    let zero = N(Node::new("0".to_string()));
    let one = N(Node::new("1".to_string()));
    let two = N(Node::new("2".to_string()));
    let three = N(Node::new("3".to_string()));
    let four = N(Node::new("4".to_string()));
    let five = N(Node::new("5".to_string()));
    let six = N(Node::new("6".to_string()));
    let seven = N(Node::new("7".to_string()));
    let eight = N(Node::new("8".to_string()));
    let nine = N(Node::new("9".to_string()));

    let first_nodes = vec![z, o, t, f, s, e, n, zero, one, two, three, four, five, six, seven, eight, nine];
    let root = root.add_children(first_nodes);

    Tree::new(Some(root))
}

fn new_trie_reversed() -> Tree<String> {
    use NodeOrT::*;
    let root = Node::new("".to_string());
    let e = N(Node::new("e".to_string()).add_children(vec![
        N(Node::new("n".to_string()).add_children(vec![
            T("o".to_string()),
            N(Node::new("i".to_string()).add_child(N(Node::new("n".to_string()))))
        ])),
        N(Node::new("e".to_string()).add_child(N(Node::new("r".to_string()).add_child(N(Node::new("h".to_string()).add_child(T("t".to_string()))))))),
        N(Node::new("v".to_string()).add_child(N(Node::new("i".to_string()).add_child(T("f".to_string())))))
    ]));
    let o = N(Node::new("o".to_string()).add_children(vec![
        N(Node::new("w".to_string()).add_child(T("t".to_string()))),
        N(Node::new("r".to_string()).add_child(N(Node::new("e".to_string()).add_child(T("z".to_string())))))
    ]));
    let r = N(Node::new("r".to_string()).add_child(N(Node::new("u".to_string()).add_child(N(Node::new("o".to_string()).add_child(T("f".to_string())))))));
    let x = N(Node::new("x".to_string()).add_child(N(Node::new("i".to_string()).add_child(T("s".to_string())))));
    let n = N(Node::new("n".to_string()).add_child(N(Node::new("e".to_string()).add_child(N(Node::new("v".to_string()).add_child(N(Node::new("e".to_string()).add_child(T("s".to_string())))))))));
    let t = N(Node::new("t".to_string()).add_child(N(Node::new("h".to_string()).add_child(N(Node::new("g".to_string()).add_child(N(Node::new("i".to_string()).add_child(T("e".to_string())))))))));
    let zero = N(Node::new("0".to_string()));
    let one = N(Node::new("1".to_string()));
    let two = N(Node::new("2".to_string()));
    let three = N(Node::new("3".to_string()));
    let four = N(Node::new("4".to_string()));
    let five = N(Node::new("5".to_string()));
    let six = N(Node::new("6".to_string()));
    let seven = N(Node::new("7".to_string()));
    let eight = N(Node::new("8".to_string()));
    let nine = N(Node::new("9".to_string()));

    let first_nodes = vec![e, o, r, x, n, t, zero, one, two, three, four, five, six, seven, eight, nine];
    let root = root.add_children(first_nodes);

    Tree::new(Some(root))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"
        .to_string();
        assert_eq!(part_1(&input), Ok("142".to_string()));
    }

    #[test]
    fn example_2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
        .to_string();
        // 29, 83, 13, 24,42, 14, 76
        assert_eq!(part_2(input), Ok("281".to_string()));
    }

    #[test]
    fn go_back_right_amount_when_not_found() {
        let input = "
bnnqzcfoneeight2hhdfkrrqzt
xfoneightsixnine6fiveseven
"
        .to_string();
        // 12, 17
        assert_eq!(part_2(input), Ok("29".to_string()));
    }   
}
