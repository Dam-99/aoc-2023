pub fn run(input: String) -> Result<String, String> {
    Ok(part_1(input).unwrap()) // + part_2.unwrap
}

fn part_1(input: String) -> Result<String, String> {
    // println!("{}", input);
    let n_lines = input.lines().count();
    let mut calibration_codes = vec![-1; n_lines];
    for (i, line) in input.lines().enumerate() { // as an improvement, it can be parallelized
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
