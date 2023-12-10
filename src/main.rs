// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Game {
    pub fn new_from_string(line: &str) -> Game {
        static GAME_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r"Game .*:").unwrap());
        let game_caps = GAME_STR.captures(line).unwrap();
        let game_str = String::from_str(game_caps.get(0).unwrap().as_str()).unwrap();
        let game_str = game_str.replace("Game ", "");
        let game_str = game_str.replace(":", "");
        let game_id = game_str.parse::<u32>().unwrap();

        // everything after the game id
        let mut red_vec = Vec::new();
        let mut blue_vec = Vec::new();
        let mut green_vec = Vec::new();

        red_vec.push(0);
        blue_vec.push(0);
        green_vec.push(0);

        static DICE_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\:(.*)").unwrap());
        static GAME_SPLIT_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r";").unwrap());
        static DICE_SPLIT_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r",").unwrap());

        static RED_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s\d+\sred").unwrap());
        static BLUE_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s\d+\sblue").unwrap());
        static GREEN_STR: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s\d+\sgreen").unwrap());
        let dice_caps = DICE_STR.captures(line).unwrap().get(0).unwrap().as_str();
        // remove first two characters
        let all_games = dice_caps[1..].to_string();

        let game_split = GAME_SPLIT_STR.split(&all_games).collect::<Vec<&str>>();
        for game in game_split {
            let dice_split = DICE_SPLIT_STR.split(game).collect::<Vec<&str>>();
            for dice in dice_split {
                if RED_STR.is_match(dice) {
                    RED_STR.captures(dice).unwrap().get(0).unwrap().as_str();
                    let mut red_dice = String::from_str(dice).unwrap();
                    red_dice = red_dice.strip_prefix(" ").unwrap().replace(" red", "");
                    let red_num = red_dice.parse::<u32>().unwrap();
                    red_vec.push(red_num);
                }
                if BLUE_STR.is_match(dice) {
                    BLUE_STR.captures(dice).unwrap().get(0).unwrap().as_str();
                    let mut blue_dice = String::from_str(dice).unwrap();
                    blue_dice = blue_dice.strip_prefix(" ").unwrap().replace(" blue", "");
                    let blue_num = blue_dice.parse::<u32>().unwrap();
                    blue_vec.push(blue_num);
                }
                if GREEN_STR.is_match(dice) {
                    GREEN_STR.captures(dice).unwrap().get(0).unwrap().as_str();
                    let mut green_dice = String::from_str(dice).unwrap();
                    green_dice = green_dice.strip_prefix(" ").unwrap().replace(" green", "");
                    let green_num = green_dice.parse::<u32>().unwrap();
                    green_vec.push(green_num);
                }
            }
        }

        Game {
            id: game_id,
            red: red_vec.iter().max().unwrap().to_owned(),
            blue: blue_vec.iter().max().unwrap().to_owned(),
            green: green_vec.iter().max().unwrap().to_owned(),
        }
    }
}


fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);

    let mut games = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let game = Game::new_from_string(&line);
        games.push(game);
    }

    let max_red: u32 = 12;
    let max_blue: u32 = 14;
    let max_green: u32 = 13;

    let mut sum_game_id: u32 = 0;
    let mut sum_power_games: u64 = 0;

    for game in &games {
        print!("{:?} ", game);
        if game.red <= max_red && game.blue <= max_blue && game.green <= max_green {
            println!("Game id: {} possible", game.id);
            sum_game_id += game.id;
        } else {
            println!("Game id: {} not possible", game.id);
        }
    }

    for game in &games {
        let power_game = game.red * game.blue * game.green;
        sum_power_games += power_game as u64;
    }

    println!("Sum of game ids: {}", sum_game_id);
    println!("Sum of power games: {}", sum_power_games);


    Ok(())
}