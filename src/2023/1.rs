pub fn run(input: String) -> Result<String, String> {
    Ok(part_1(input).unwrap()) // + part_2.unwrap
}

fn part_1(input: String) -> Result<String, String> {
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
    println!("Done: {}", result);
    Ok(result.to_string())
}

#[derive(Clone)]
struct Node<T: Clone> {
    value: T,
    children: Vec<Node<T>>,
}
impl<T: Clone> Node<T> {
    fn new(el: T) -> Node<T> {
        let node = Node::<T> {
            value: el,
            children: Vec::<Node<T>>::new(),
        };
        node
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
        self.children.len() > 0
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
fn part_2(input: String) -> Result<String, String> {
    let n_lines = input.lines().count();
    let mut calibration_codes = vec![-1; n_lines];
    let letter_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; //tree
    //for when navigating the chars? then for the last one it can walk it backwards by having
    //pointers to all of them
    for (i, line) in input.lines().enumerate() {
        let digits = line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>();
        let mut number = digits.first().unwrap().to_string();
        number.push_str(&digits.last().unwrap().to_string());
        let number = number.parse::<i32>().unwrap();
        //println!("{}",number);
        calibration_codes[i] = number;
    }
    //calibration_codes.iter().for_each(|n| print!("{} ", n));
    let result: i32 = calibration_codes.iter().sum();
    println!("Done: {}", result);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
".to_string();
        assert_eq!(part_1(input), Ok("142".to_string()));
    }

    #[test]
    fn example_2() { // PART 2 probs easier solved with common substring problem
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
".to_string();
        // 29, 83, 13, 24,42, 14, 76
        assert_eq!(part_2(input), Ok("281".to_string()));
    }
}
