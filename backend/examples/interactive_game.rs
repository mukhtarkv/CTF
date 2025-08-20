use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor};

// Import your game types
use ctf_backend::game::{GameState, Move};

fn main() {
    println!("🎮 Interactive Game Test");
    println!("Controls:");
    println!("Player 0: W/A/S/D (Up/Left/Down/Right)");
    println!("Player 1: Arrow Keys");
    println!("Player 2: F/G/H/T (Up/Left/Down/Right)");
    println!("Player 3: J/K/L/I (Up/Left/Down/Right)");
    println!("Commands: 'q' to quit, 'r' to reset, 'p' to print");
    println!("Press any key to start...\n");
    
    let _stdin = io::stdin();
    let mut _stdout = io::stdout().into_raw_mode().unwrap();

    let mut game = GameState::<4>::new();
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    game.pretty_print();
    io::stdout().flush().unwrap();

    for key in io::stdin().keys() {
        match key.unwrap() {
            Key::Char('q') => {
                println!("👋 Quitting...");
                break;
            }
            Key::Char('r') => {
                game = GameState::<4>::new();
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                println!("🔄 Game reset!");
                game.pretty_print();
                io::stdout().flush().unwrap();
            }
            Key::Char('p') => {
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                game.pretty_print();
                io::stdout().flush().unwrap();
            }
            key => {
                let moves = parse_key_to_moves(key);
                game.step(moves);
                // Clear screen and reprint after each move
                print!("{}{}", clear::All, cursor::Goto(1, 1));
                game.pretty_print();
                io::stdout().flush().unwrap();
            }
        }
    }
}

fn parse_key_to_moves(key: Key) -> [Move; 4] {
    let mut moves = [Move::Stay; 4];
    
    match key {
        // Player 0: WASD
        Key::Char('w') => moves[0] = Move::Up,
        Key::Char('a') => moves[0] = Move::Left,
        Key::Char('s') => moves[0] = Move::Down,
        Key::Char('d') => moves[0] = Move::Right,
        
        // Player 1: Arrow keys
        Key::Up => moves[1] = Move::Up,
        Key::Left => moves[1] = Move::Left,
        Key::Down => moves[1] = Move::Down,
        Key::Right => moves[1] = Move::Right,
        
        // Player 2: FGHT
        Key::Char('f') => moves[2] = Move::Up,
        Key::Char('g') => moves[2] = Move::Left,
        Key::Char('h') => moves[2] = Move::Down,
        Key::Char('t') => moves[2] = Move::Right,
        
        // Player 3: JKLI
        Key::Char('j') => moves[3] = Move::Up,
        Key::Char('k') => moves[3] = Move::Left,
        Key::Char('l') => moves[3] = Move::Down,
        Key::Char('i') => moves[3] = Move::Right,
        
        _ => {} // All players stay
    }
    
    moves
}
