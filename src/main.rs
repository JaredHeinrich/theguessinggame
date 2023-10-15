use std::cmp::Ordering;
use std::process::exit;
use rand::Rng;
use std::process::Command;
use colored::{Colorize, ColoredString};
use std::io::{self, Write};


trait ColorizeString{
    fn colorize(&self, t: (u8, u8, u8)) -> ColoredString;
}
impl ColorizeString for str {
    fn colorize(&self, t:(u8, u8, u8) ) -> ColoredString {
        return self.truecolor(t.0, t.1, t.2);
    }
}

pub const CAP_BOTTOM: u32 = 1;
pub const NUMBER_OF_DIFFICULTIES: usize = 5;

pub const DIFFICULTIES_TOP: [u32; NUMBER_OF_DIFFICULTIES] = [
    10,
    100,
    1000,
    10000,
    100000
];

pub const DIFFICULTIES_NAME: [&str; NUMBER_OF_DIFFICULTIES] = [
    "easy",
    "medium",
    "hard",
    "expert",
    "extreme"
];

pub const DIFFICULTIES_COL: [(u8, u8, u8); NUMBER_OF_DIFFICULTIES] = [
    (0, 170, 0),
    (128, 255, 0),
    (255, 255, 0),
    (255, 128, 0),
    (255, 0, 0)
];

pub const COL_TO_BIG: (u8, u8, u8) = (255, 85, 0);
pub const COL_TO_SMALL: (u8, u8, u8) = (25, 178, 255);
pub const COL_ERROR: (u8, u8, u8) = (255, 128, 128);
pub const COL_MAIN: (u8, u8, u8) = (77, 136, 255);
pub const COL_WHITE: (u8, u8, u8) = (255, 255, 255);
pub const COL_WIN: (u8, u8, u8) = (0, 204, 0);

fn main() {
    loop{
        print_main_menu();
        println!("");
        let difficulty = get_difficulty();
        run_game(difficulty);
    }//loop
}

fn get_difficulty() -> usize {
    loop {
        let input: String = get_user_input(); 
        let input = input.trim();
        println!("{}", input);
        if input.eq("menu") || input.eq("m") {
            print_main_menu();
            println!("{}", "Already on main menu".colorize(COL_ERROR));
            continue;
        }

        let mut input: u32 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                print_main_menu();
                println!("{}: {}","unknown command".colorize(COL_ERROR).bold(), input.colorize(COL_ERROR).bold());
                continue;
            },
        };
        input = input - 1;
        let input: usize = input as usize;
        if input >= NUMBER_OF_DIFFICULTIES {
            print_main_menu();
            println!("{}: {}","unknown command".colorize(COL_ERROR).bold(), format!("{}", input).colorize(COL_ERROR).bold());
            continue;
        }
        return input;
    }
}
fn run_game(difficulty: usize) {
    loop {
        clear_terminal();
        println!("{} {} {} {}{}", "Guess a number between".colorize(COL_MAIN), CAP_BOTTOM.to_string().colorize(COL_WHITE).bold(), "and".colorize(COL_MAIN), DIFFICULTIES_TOP[difficulty].to_string().colorize(COL_WHITE).bold(), "!".colorize(COL_MAIN));
        let secret_number = rand::thread_rng().gen_range(CAP_BOTTOM..=DIFFICULTIES_TOP[difficulty]);
        let mut won: bool = false;
        loop {
            let guess: String = get_user_input(); 
            let guess = guess.trim();
            if guess.eq("menu") || guess.eq("m") {
                return;
            }
            if guess.eq("replay") || guess.eq("r") {
                break;
            }
            let guess: u32 = match guess.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", "Please type a number".colorize(COL_ERROR).bold());
                    continue;
                },
            };
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("{}", "Too small".colorize(COL_TO_SMALL)), 
                Ordering::Equal => {println!("{}", "You win!".colorize(COL_WIN).bold()); won = true; break;},  
                Ordering::Greater => println!("{}", "Too big".colorize(COL_TO_BIG)), 
            }
        }

        if won {
            println!("Choose: {} | {} | {}",
                     "[r]eplay".colorize(COL_WHITE).bold(),
                     "[m]enu".colorize(COL_WHITE).bold(),
                     "[q]uit".colorize(COL_WHITE).bold()
                     );
            let mut replay; 
            loop {
                replay = get_user_input();
                replay = replay.trim().to_lowercase();
                match replay.as_str() {
                    "r" => {
                        clear_terminal();
                        break;
                    }
                    "replay" => {
                        clear_terminal();
                        break;
                    }
                    "m" => {
                        clear_terminal();
                        return;
                    }
                    "menu" => {
                        clear_terminal();
                        return;
                    }
                    _ => {
                        println!("{}: {}", "wrong command".colorize(COL_ERROR), format!("{}", replay).colorize(COL_ERROR));
                        continue;
                    }
                }
            }
        }
    }
}

fn clear_terminal() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
        Command::new("clear").status().unwrap();
    }
}

fn get_user_input() -> String {
    print!(">>> ");
    match io::stdout().flush() {
        Ok(_) => {},
        Err(_) => {println!("stdout flush error");
        }
    };
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        if input.trim().eq("quit") || input.trim().eq("q") {
            clear_terminal();
            println!("{}", "bye <3".colorize(COL_MAIN));
            exit(0);
        }
    return input;
}

fn print_main_menu() {
    clear_terminal();
    println!("{}", "
╔╦╗┬ ┬┌─┐ ╔═╗┬ ┬┌─┐┌─┐┌─┐┬┌┐┌┌─┐ ╔═╗┌─┐┌┬┐┌─┐
 ║ ├─┤├┤  ║ ╦│ │├┤ └─┐└─┐│││││ ┬ ║ ╦├─┤│││├┤ 
 ╩ ┴ ┴└─┘ ╚═╝└─┘└─┘└─┘└─┘┴┘└┘└─┘ ╚═╝┴ ┴┴ ┴└─┘
             ".colorize(COL_MAIN));
    println!("{}", "Difficulties:".colorize(COL_WHITE).bold());
    for i in 0..NUMBER_OF_DIFFICULTIES {
        println!("{}: {} ({}-{})",
        format!("{}", i+1).colorize(DIFFICULTIES_COL[i]),
        DIFFICULTIES_NAME[i].colorize(DIFFICULTIES_COL[i]),
        format!("{}", CAP_BOTTOM).colorize(DIFFICULTIES_COL[i]),
        format!("{}", DIFFICULTIES_TOP[i]).colorize(DIFFICULTIES_COL[i])
        );
    }//for
    println!("");
    println!("{}", "Commands:".colorize(COL_WHITE).bold());
    println!("{}: replay", "[r]eplay".colorize(COL_WHITE).bold());
    println!("{}: return to main menu", "[m]enu".colorize(COL_WHITE).bold());
    println!("{}: quit the game", "[q]uit".colorize(COL_WHITE).bold());
}
