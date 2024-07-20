use beep::beep;
use std::io::prelude::*;
use std::{io::Write, thread::sleep, thread::JoinHandle, time::Duration};

mod detect;
mod display;
mod game_file;

const DEFAULT_BOARD_SIZE: usize = 8;

// https://en.wikipedia.org/wiki/Portable_Game_Notation

/*
🨞 🨤 🨀
*/
struct Tile {
    row: u8,
    col: u8,
}

fn main() {
    let single_player_mode: bool = false;

    // Spin up threads
    let loading_thread = splash_screen();

    // Join the threads
    loading_thread.join().unwrap();

    //let musicThread = music();
    //musicThread.join().unwrap();

    println!("\nChoose an option:");
    println!("New Game...      (enter \"1\")");
    println!("Continue Game... (enter \"2\")");
    println!("Tutorial...      (enter \"3\")\n");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let new_board = display::GameBoard::ctor(None, None);

    println!("Your input was {}", input);
    match input.trim() {
        "1" => display::GameBoard::show_board(&new_board),
        "2" => display::GameBoard::show_board(&new_board),
        "3" => display::GameBoard::tutorial(),
        _ => println!("Sorry, I didn't quite understand that..."),
    };
}

fn splash_screen() -> JoinHandle<()> {
    std::thread::spawn(|| {
        let mut stdout = std::io::stdout();
        println!("   ████           █");
        println!("  █  █████████████");
        println!(" █     █████████");
        println!(" █     █  █");
        println!("  ██  █  ██        ███  ████               ████      ████");
        println!("     █  ███         ████ ████ █   ███     █ ████ █  █ ████ █");
        println!("    ██   ██          ██   ████   █ ███   ██  ████  ██  ████");
        println!("    ██   ██          ██         █   ███ ████      ████");
        println!("    ██   ██          ██        ██    ███  ███       ███");
        println!("    ██   ██          ██        ████████     ███       ███");
        println!("     ██  ██          ██        ███████        ███       ███");
        println!("      ██ █      █    ██        ██        ████  ██  ████  ██");
        println!("       ███     █     ███       ████    ██ ████ █  █ ████ █");
        println!("        ███████       ███       ███████    ████      ████");
        println!("          ███                    █████\n");

        for ii in 0..100 {
            print!("\rEbi Sadeghi\t\t\t\t\tLoading {}%...", ii);
            stdout.flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        println!();
    })
}

/*
fn music() -> JoinHandle<()> {
    let this_thread = std::thread::spawn(|| {
        beep(440);
        std::thread::sleep(std::time::Duration::from_millis(500));
        beep(880);
        std::thread::sleep(std::time::Duration::from_millis(500));
        beep(0);
    });

    return this_thread;
}

fn quit_program() {}
*/
