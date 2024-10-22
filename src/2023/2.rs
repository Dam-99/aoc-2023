pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
    Ok("Day 2 completed".to_string())
}

#[derive(Clone)]
struct Blocks { 
    red: u32,
    green: u32,
    blue: u32
}
fn part_1(input: &str) -> Result<String, String> {
    let mut possible_games = vec![0; input.lines().count()];
    let max_colours = Blocks { red: 12, green: 13, blue: 14 };
    for (game_id, game) in input.lines().enumerate() { // parallelize to make it beter
        let mut game_split = game.split(':');
        game_split.next(); // skip 'Game: '
        let pulls = game_split.next().unwrap().split(';');
        let mut impossible = false;
        for pull in pulls {
            let blocks_colour = pull.split(',');
            for block in blocks_colour {
                let mut block_split = block.split(' ');
                block_split.next(); // skip first space
                let num = block_split.next().unwrap().parse::<u32>().unwrap();
                let colour = block_split.next().unwrap();
                let max_colour = match colour {
                    "red" => max_colours.red,
                    "green" => max_colours.green,
                    "blue" => max_colours.blue,
                    _ => return Err("Unsupported colour".to_string())
                };
                if num > max_colour { // num > max_colours[colour] doesn't work
                    impossible = true;
                    break;
                }
            }
            if impossible { break; }
        }
        if !impossible { possible_games[game_id] = game_id as i32 + 1 }
    }
    let result: i32 = possible_games.iter().sum();
    Ok(format!("{result}"))
}

fn part_2(input: String) -> Result<String, String> {
    let mut games_mins = vec![Blocks { red: u32::MAX, green: u32::MAX, blue: u32::MAX }; input.lines().count()];
    for (game_id, game) in input.lines().enumerate() {
        let mut game_split = game.split(':');
        game_split.next();
        let pulls = game_split.next().unwrap().split(';');
        let mut max_colours = Blocks { red: 0, green: 0, blue: 0 };
        for pull in pulls {
            let blocks_colour = pull.split(',');
            for block in blocks_colour {
                let mut block_split = block.split(' ');
                block_split.next();
                let num = block_split.next().unwrap().parse::<u32>().unwrap();
                let colour = block_split.next().unwrap();
                macro_rules! check_max_blocks {
                    ( $c:ident ) => {
                        if num > max_colours.$c {
                            max_colours.$c = num;
                        }
                    }
                }
                match colour {
                    "red" => check_max_blocks!(red),
                    "green" => check_max_blocks!(green),
                    "blue" => check_max_blocks!(blue),
                    _ => return Err("Unsupported colour".to_string())
                }
            }
        }
        games_mins[game_id] = max_colours;
    }
    let games_powers = games_mins.iter().map(|blocks| blocks.red * blocks.green * blocks.blue);
    let result: u32 = games_powers.sum();
    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();
        assert_eq!(part_1(&input), Ok("8".to_string()));
    }

    #[test]
    fn example_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();
        let part2res = part_2(input);
        assert_eq!(part2res, Ok("2286".to_string()));
    }
}
