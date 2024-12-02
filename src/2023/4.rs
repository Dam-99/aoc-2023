use std::collections::HashSet;

pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
    Ok("Day 3 completed".to_string())
}

fn part_1(input: &str) -> Result<String, String> {
    let mut result: u32 = 0;
    for l in input.lines() {
        let mut l_split = l.split(':');
        l_split.next(); // skip line header
        let lottery_nums = l_split.next().unwrap().trim();
        let mut split = lottery_nums.split('|');
        let winning = split.next().unwrap();
        let played = split.next().unwrap();
        let mut winning_set = winning.split(' ').filter(|x| *x != "").collect::<HashSet<_>>();
        let mut played_set = played.split(' ').filter(|x| *x != "").collect::<HashSet<_>>();
        let mut count: u32 = 0;
        for plyd in played_set {
            if winning_set.contains(plyd) {
                count += 1
            }
        }
        let base: u32 = 2;
        result += match count { 0 => 0, x => base.pow(x-1) }
    }
    Ok(result.to_string())
}

fn part_2(input: String) -> Result<String, String> {
    Ok("not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
        .to_string();
        assert_eq!(part_1(&input), Ok("13".to_string()))
    }
}
