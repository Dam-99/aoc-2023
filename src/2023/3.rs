pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
    Ok("Day 3 completed".to_string())
}

#[derive(Debug)]
struct EnginePart {
    number: String, // ik, stupid that the number is a string but it's easier to concat
    line: u32,
    col: u32,
    len: u32,
    // symbol or its position ???
}

fn part_1(input: &str) -> Result<String, String> {
    let line_len = input.lines().next().unwrap().len();
    let n_lines = input.lines().count();
    // View input as a 2d matrix so i can work with indeces
    // based on https://stackoverflow.com/a/36376568
    // 1d array + math to do this without this crazy setup yeah yeah ik

    // Base 1d array
    let input_lines: Vec<_> = input.lines()
        .map(|l|
            l.chars()
                // .map(|c| c.to_string())
                .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>() // dunno if needed
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    // Vector of 'width' elements slices
    let input_grid_base: Vec<&[_]> = input_lines.as_slice().chunks(line_len).collect::<Vec<_>>();
    // Final 2d array `&mut [&mut [_]]`
    let input_grid: &[&[_]] = input_grid_base.as_slice();
    let mut parts: Vec<EnginePart> = vec![];
    for i in 0..n_lines {
        let mut num = "".to_string();
        let mut skip = 0;
        for j in 0..line_len {
            if skip > 0 {
                skip-=1;
                continue;
            }
            if input_grid[i][j].is_numeric() {
                for offset in 0..line_len-j {
                    if input_grid[i][j+offset].is_numeric() {
                        // technically could be more memory efficient with this
                        // https://www.reddit.com/r/rust/comments/eanwkm/comment/fauqqdx/
                        num += input_grid[i][j+offset].to_string().as_str();
                    }
                    else {
                        skip = offset-1;
                        break;
                    }
                }
                // this whole part was supposed to be out of this loop
                // (that's why EnginePart has 'len' and main reason i did the 2d array)
                // ended up not being needed
                let x_range = match i {
                    0 => i..=i+1,
                    y if y == n_lines-1 => i-1..=i,
                    _ => i-1..=i+1
                };
                let y_range = match j {
                    0 => j..=j+1+skip,
                    x if x == line_len-1 => j-1..=j,
                    _ => j-1..=j+1+skip
                };
                // don't want to use a crate so... double loop
                for x in x_range {
                    for y in y_range.clone() {
                        if x == i && (j..=j+skip).contains(&y)  { continue; }
                        let curr = input_grid[x][y];
                        if curr != '.' && !curr.is_alphanumeric() {
                            parts.push(EnginePart { number: num.clone(), line: i as u32, col: j as u32, len: num.len() as u32 });
                            break;
                        }
                    }
                }
                num = "".to_string();
            }
        }
    }
    let result= parts.iter().map(|ep| ep.number.parse::<u32>().unwrap());
        let result: u32 = result.sum();
    Ok(format!("{result}"))
}

#[derive(Debug)]
struct Gear {
    num1: Option<u32>,
    num2: Option<u32>,
    coords: Coords,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coords {
    x: u32,
    y: u32,
}
use std::collections::HashMap;

// baso the same as part 1
fn part_2(input: String) -> Result<String, String> {
    let line_len = input.lines().next().unwrap().len();
    let n_lines = input.lines().count();

    // Base 1d array
    let input_lines: Vec<_> = input
        .lines()
        .map(|l| {
            l.chars()
                // .map(|c| c.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>() // dunno if needed
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    // Vector of 'width' elements slices
    let input_grid_base: Vec<&[_]> = input_lines.as_slice().chunks(line_len).collect::<Vec<_>>();
    // Final 2d array `&mut [&mut [_]]`
    let input_grid: &[&[_]] = input_grid_base.as_slice();
    let mut gears: HashMap<Coords, Gear> = HashMap::new();
    for i in 0..n_lines {
        let mut num = "".to_string();
        let mut skip = 0;
        for j in 0..line_len {
            if skip > 0 {
                skip -= 1;
                continue;
            }
            if input_grid[i][j].is_numeric() {
                for offset in 0..line_len - j {
                    if input_grid[i][j + offset].is_numeric() {
                        num += input_grid[i][j + offset].to_string().as_str();
                    } else {
                        skip = offset - 1;
                        break;
                    }
                }
                let x_range = match i {
                    0 => i..=i + 1,
                    y if y == n_lines - 1 => i - 1..=i,
                    _ => i - 1..=i + 1,
                };
                let y_range = match j {
                    0 => j..=j + 1 + skip,
                    x if x == line_len - 1 => j - 1..=j,
                    _ => j - 1..=j + 1 + skip,
                };
                for x in x_range {
                    for y in y_range.clone() {
                        if x == i && (j..=j + skip).contains(&y) {
                            continue;
                        }
                        let curr = input_grid[x][y];
                        if curr == '*' {
                            let curr_coords = Coords { x: x as u32, y: y as u32 };
                            let num_parsed = num.parse::<u32>().unwrap();
                            let opt_gear = gears.get(&curr_coords);
                            match opt_gear {
                                None => gears.insert(curr_coords,
                                    Gear { coords: curr_coords, num1: Some(num_parsed), num2: None, },),
                                Some(g) => gears.insert(curr_coords,
                                    Gear { coords: g.coords, num1: g.num1, num2: Some(num_parsed), },),
                            };
                            break;
                        }
                    }
                }
                num = "".to_string();
            }
        }
    }
    let filtered_gears = gears.values().filter(|g| g.num2.is_some());
    let result: u32 = filtered_gears
        .map(|g| g.num1.unwrap() * g.num2.unwrap())
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        assert_eq!(part_1(&input), Ok("4361".to_string()));
    }

    #[test]
    fn example_2() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();
        let part2res = part_2(input);
        assert_eq!(part2res, Ok("467835".to_string()));
    }
}
