pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    Ok("Day 2 completed".to_string())
}

struct MaxBlocks { 
    red: i32,
    green: i32,
    blue: i32
}
fn part_1(input: &str) -> Result<String, String> {
    let mut possible_games = vec![0; input.lines().count()];
    let max_colours = MaxBlocks { red: 12, green: 13, blue: 14 };
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
                let num = block_split.next().unwrap().parse::<i32>().unwrap();
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
}
