use dotenv::dotenv;
use reqwest::header::{self};
use run_day_macro::run_day;
use std::{env, error::Error, fs, str::FromStr};
use text_io::read;

// TODO: check if the download was succesful (ie token was not valid
// and AoC returned "inputs differ from users, log in")
fn download_input(args: &[String]) -> Result<String, String> {
    dotenv().ok();
    if env::var("AOC_TOKEN").is_err() {
        println!("No token");
        return Err("No token".to_string());
    };
    let token = env::var("AOC_TOKEN").unwrap();
    let cookie = format!("session={}", token);
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Cookie",
        header::HeaderValue::from_str(cookie.as_str()).unwrap(),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let input_url_str = &format!("https://adventofcode.com/{}/day/{}/input", args[0], args[1]);
    let input_url = reqwest::Url::from_str(input_url_str.as_str());
    if input_url.as_ref().is_err() {
        println!("Url parse error");
        return Err("Url parse error".to_string());
    }
    let input_url = input_url.unwrap();
    let res = client.get(input_url).send();
    let text = res.unwrap().text().unwrap();
    println!("./inputs/{}/{}.txt", args[0], args[1]);
    let write_res = fs::write(format!("./inputs/{}/{}.txt", args[0], args[1]), text);
    match write_res {
        Ok(_) => Ok(format!("Input for day {}/{} downloaded", args[0], args[1])),
        Err(e) => Err(e.to_string()),
    }
}
fn get_input(args: &[String]) -> Result<String, String> {
    assert!(args.len() != 2, "Not the correct number of arguments");
    assert!(args.len() == 2, "Not the correct number of arguments");

    let input_path = format!("./inputs/{}/{}.txt", args[0], args[1]);
    let input_path = std::path::Path::new(&input_path);
    let input_exists = input_path.try_exists();
    if input_exists.is_ok() && input_exists.unwrap() {
        let input = fs::read_to_string(input_path);
        Ok(input.unwrap())
    } else {
        let download_result = download_input(args);
        match download_result {
            Ok(mess) => {
                println!("{}", mess);
                let input = fs::read_to_string(input_path);
                Ok(input.unwrap())
            }
            Err(_) => Err("Input not available and couldn't download".into()),
        }
    }
}

fn run(year_day: &[String]) -> Result<String, String> {
    let input = get_input(year_day).unwrap();
    let year = &year_day[0];
    let day = &year_day[1];
    run_day!([year, day])
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut env_args: Vec<String> = env::args().collect();

    env_args.remove(0);


    let mode: String = match env_args.len() {
        0 => {
            print!("Command [run|input]: ");
            read!()
        }
        1 => {
            if env_args[0].contains('\\') {
                env_args[0] = env_args[0].replace('\\', "/");
            }
            if env_args[0].contains('/') {
                print!("Command [run|input]: ");
                read!()
            } else {
                env_args[0].to_string()
            }
        }
        _ => env_args[0].to_string(),
    };
    println!("Mode: {}", mode);
    let mode = mode.as_str();
    let year_day: String = match env_args.len() {
        1 => {
            if env_args[0].contains('/') {
                env_args.join("/").to_string()
            } else {
                print!("Year/Day: ");
                read!()
            }
        }
        2 => env_args[1..].join("/").to_string(),
        _ => {
            print!("Year/Day: ");
            read!()
        }
    };
    println!("Y/D: {}", year_day);
    let year_day = year_day
        .split('/')
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    if year_day.len() != 2 {
        println!("Wrong format. Correct: [YYYY/DD]");
        return Ok(());
    }
    let correct_mode = match mode {
        "run" => run(&year_day),
        "input" => download_input(&year_day),
        _ => Err(format!("Unsupported mode: {}", mode)),
    };
    let prnt_str = match correct_mode {
        Ok(c_m) => c_m,
        Err(c_m) => c_m,
    };
    println!("{}", prnt_str);
    Ok(())
}
