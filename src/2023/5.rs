use std::{collections::HashMap, ops::RangeInclusive};

pub fn run(input: String) -> Result<String, String> {
    println!("Part 1: {}", part_1(&input).unwrap());
    println!("Part 2: {}", part_2(input).unwrap());
    Ok("Day 5 completed".to_string())
}

#[derive(Debug)]
struct AlmanacMap<'a> {
    dest_name: &'a str,
    map: Vec<AlmanacRange>
}

#[derive(Debug)]
struct AlmanacRange {
    source: u64,
    dest: u64,
    len: u64
}

fn part_1(input: &str) -> Result<String, String> {
    let mut input_chunks = input.split("\n\n");
    input_chunks.next();
    let seeds = input.lines().next().unwrap().split(':').next_back().unwrap().split(' ').collect::<Vec<_>>();
    let seeds = &seeds[1..];
    let mut maps: HashMap<_, AlmanacMap> = HashMap::new();
    // println!("seeds: {seeds:?}");
    for ch in input_chunks {
        // println!("chunk: {ch}");
        let map_name = ch.lines().next().unwrap().split(' ').next().unwrap();
        let mut cur_map = vec![];
        let source = map_name.split('-').next().unwrap();
        let dest = map_name.split('-').next_back().unwrap();
        // println!("{source} {dest}");
        for (i, range_line) in ch.lines().enumerate() {
            if i == 0 { continue; }
            let range_split: Vec<_> = range_line.split(' ').collect();
            let dest_start = range_split[0];
            let source_start = range_split[1];
            let len = range_split[2];
            cur_map.push(AlmanacRange { source: source_start.parse().unwrap(), dest: dest_start.parse().unwrap(), len: len.parse().unwrap() });
        }
        maps.insert(source, AlmanacMap { dest_name: dest, map: cur_map });
    }
    // println!("map_name {next_map_name}");
    // println!("maps: {maps:?}");
    let mut locations = vec![u64::MAX; seeds.len()];
    for (i, seed) in seeds.iter().enumerate() {
        let mut next_map_name = "seed";
        let mut search_val = seed.parse().unwrap();
        // print!("seed {seed} ");
        loop {
            let cur_map_opt = maps.get(next_map_name);
            // println!("cur_map_opt: {cur_map_opt:?}");
            // println!("-> {next_map_name} {search_val} ");
            if let Some(cur_map) = cur_map_opt {
                // println!("map_name {next_map_name}");
                next_map_name = cur_map.dest_name;
                // println!("{:?}", cur_map.map);
                for r in cur_map.map.as_slice() {
                    // println!("{r:?}");
                    // println!("{}-{} -> {}-{}", r.source, r.source + r.len - 1, r.dest, r.dest + r.len - 1);
                    let search_range = r.source..r.source + r.len;
                    // println!("{search_range:?}");
                    // println!("{}-{} -> {}-{}", r.source, r.source + r.len - 1, r.dest, r.dest + r.len - 1);
                    if search_range.contains(&search_val) {
                        let offset = r.source + r.len - search_val;
                        search_val = r.dest + r.len - offset; // no need to update if it's not one of the
                        // println!("source+len-search_val = {offset} => dest+offset = {search_val}");
                        // mapped ranges
                        break;
                    }
                }
            }
            // else { println!("maps.len {}", maps.len()); println!("break"); break; }
            else { break; }
        }
        // println!();
        locations[i] = search_val;

    }
    let result = locations.iter().min().unwrap();
    println!("{locations:?}");
    Ok(result.to_string())
}

#[derive(Debug)]
struct SeedsRange {
    start: u64,
    len: u64
}

fn part_2(input: String) -> Result<String, String> {
    let mut input_chunks = input.split("\n\n");
    input_chunks.next();
    let seed_ranges = input.lines().next().unwrap().split(':').next_back().unwrap().split(' ').collect::<Vec<_>>();
    let seed_ranges = &seed_ranges[1..];
    let mut seed_range_start = "";
    let mut seeds: Vec<SeedsRange> = vec![];
    let mut check_amount = 0_u64;
    for (i, num) in seed_ranges.iter().enumerate() {
        if i % 2 == 0 {
            seed_range_start = num;
        }
        else {
            seeds.push(SeedsRange { start: seed_range_start.parse().unwrap(), len: num.parse().unwrap() }); // all the numbers don't fit in memory
        check_amount += num.parse::<u64>().unwrap();
        }
    }
    println!("amount {check_amount}");
    let mut maps: HashMap<_, AlmanacMap> = HashMap::new();
    // println!("seeds: {seeds:?}");
    for ch in input_chunks {
        // println!("chunk: {ch}");
        let map_name = ch.lines().next().unwrap().split(' ').next().unwrap();
        let mut cur_map = vec![];
        let source = map_name.split('-').next().unwrap();
        let dest = map_name.split('-').next_back().unwrap();
        // println!("{source} {dest}");
        for (i, range_line) in ch.lines().enumerate() {
            if i == 0 { continue; }
            let range_split: Vec<_> = range_line.split(' ').collect();
            let dest_start = range_split[0];
            let source_start = range_split[1];
            let len = range_split[2];
            cur_map.push(AlmanacRange { source: source_start.parse().unwrap(), dest: dest_start.parse().unwrap(), len: len.parse().unwrap() });
        }
        maps.insert(source, AlmanacMap { dest_name: dest, map: cur_map });
    }
    // println!("map_name {next_map_name}");
    // println!("maps: {maps:?}");
    let mut location = u64::MIN;
    for cur_seeds in seeds.iter() {
        print!(" --- {cur_seeds:?} ");
        for seed in cur_seeds.start..(cur_seeds.start + cur_seeds.len) { // too slow because there are many, parallelism would probs help
            // but it's better to rewrite it to just check the bounds in the maps
            // cur.start >= map_range.start && cur.start + cur.len >= map_range.start + map_range.len
            // => create new [cur.start + (cur.start + cur.len) - (map_range.start + map_range.len), cur.start + cur.len] to check (i think)
            // NOPE: keeping track of the maps that created the splits and restarting the search is too much work
            // DONE: even better: only store the min; even if it takes a lot, at least it won't run out of memory
            let mut next_map_name = "seed";
            let mut search_val = seed;
            // print!("seed {seed}");
            loop {
                // print!("search_val {search_val} ");
                let cur_map_opt = maps.get(next_map_name);
                // println!("cur_map_opt: {cur_map_opt:?}");
                // println!("-> {next_map_name} {search_val} ");
                if let Some(cur_map) = cur_map_opt {
                    // println!("map_name {next_map_name}");
                    next_map_name = cur_map.dest_name;
                    // println!("{:?}", cur_map.map);
                    for r in cur_map.map.as_slice() {
                        // println!("{r:?}");
                        // println!("{}-{} -> {}-{}", r.source, r.source + r.len - 1, r.dest, r.dest + r.len - 1);
                        let search_range = r.source..r.source + r.len;
                        // println!("{search_range:?}");
                        // println!("{}-{} -> {}-{}", r.source, r.source + r.len - 1, r.dest, r.dest + r.len - 1);
                        if search_range.contains(&search_val) {
                            let offset = r.source + r.len - search_val;
                            search_val = r.dest + r.len - offset; // no need to update if it's not one of the
                            // println!("source+len-search_val = {offset} => dest+offset = {search_val}");
                            // mapped ranges
                            break;
                        }
                    }
                }
                // else { println!("maps.len {}", maps.len()); println!("break"); break; }
                else { break; }
            }
            // println!();
            location = if search_val < location { search_val } else { location };

        }
    }
    println!();
    let result = location;
    println!("{location:?}");
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_string();
        assert_eq!(part_1(&input), Ok("35".to_string()))
    }

    #[test]
    fn example_2() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .to_string();
        assert_eq!(part_2(input), Ok("46".to_string()))
    }
}
