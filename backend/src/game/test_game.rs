use std::io::{self, Read};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[cfg(test)]
mod cli_test {
    use super::*;

    #[test]
    fn interactive_test() {
        println!("Interactive Game Test");
        println!("Controls:");
        println!("Player 0: W/A/S/D (Up/Left/Down/Right)");
        println!("Player 1: Arrow Keys");
        println!("Player 2: F/G/H/T (Up/Left/Down/Right)");
        println!("Player 3: H/K/L/I (Up/Left/Down/Right)");
        println!("Press 'q' to quit, 'r' to reset, 'p' to print game state");
        println!("Press any key to start...");

        // Wait for initial keypress
        let _stdin = io::stdin();
        let mut _stdout = io::stdout().into_raw_mode().unwrap();

        let mut game = GameState::<4>::new();
        game.pretty_print();

        // Main game loop
        for key in io::stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    println!("Quitting...");
                    break;
                }
                Key::Char('r') => {
                    game = GameState::<4>::new();
                    println!("Game reset!");
                    game.pretty_print();
                    continue;
                }
                Key::Char('p') => {
                    game.pretty_print();
                    continue;
                }
                key => {
                    let moves = parse_key_to_moves(key);
                    game.step(moves);
                    // Optionally print after each move (comment out if too verbose)
                    // game.pretty_print();
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

            // Player 3: JKLI (note: H conflicts with player 2, using JKLI instead)
            Key::Char('j') => moves[3] = Move::Up,
            Key::Char('k') => moves[3] = Move::Left,
            Key::Char('l') => moves[3] = Move::Down,
            Key::Char('i') => moves[3] = Move::Right,

            _ => {} // All players stay if unrecognized key
        }

        moves
    }
}

// Alternative version that reads line-by-line instead of raw key input
#[cfg(test)]
mod line_input_test {
    use super::*;

    #[test]
    fn line_based_test() {
        println!("Line-based Interactive Test");
        println!("Enter moves as a string (e.g., 'wd' moves player 0 up and right)");
        println!("Available keys:");
        println!("Player 0: wasd | Player 1: ↑←↓→ (use 8462) | Player 2: fght | Player 3: jkli");
        println!("Special: 'q' to quit, 'r' to reset, 'p' to print");

        let mut game = GameState::<4>::new();
        game.pretty_print();

        loop {
            println!("Enter move(s): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if input == "q" {
                break;
            } else if input == "r" {
                game = GameState::<4>::new();
                println!("Game reset!");
                game.pretty_print();
                continue;
            } else if input == "p" {
                game.pretty_print();
                continue;
            }

            // Process each character in the input
            for ch in input.chars() {
                let moves = parse_char_to_moves(ch);
                game.step(moves);
            }

            game.pretty_print();
        }
    }

    fn parse_char_to_moves(ch: char) -> [Move; 4] {
        let mut moves = [Move::Stay; 4];

        match ch {
            // Player 0: WASD
            'w' => moves[0] = Move::Up,
            'a' => moves[0] = Move::Left,
            's' => moves[0] = Move::Down,
            'd' => moves[0] = Move::Right,

            // Player 1: 8462 (representing arrow keys)
            '8' => moves[1] = Move::Up,
            '4' => moves[1] = Move::Left,
            '6' => moves[1] = Move::Down,
            '2' => moves[1] = Move::Right,

            // Player 2: FGHT
            'f' => moves[2] = Move::Up,
            'g' => moves[2] = Move::Left,
            'h' => moves[2] = Move::Down,
            't' => moves[2] = Move::Right,

            // Player 3: JKLI
            'j' => moves[3] = Move::Up,
            'k' => moves[3] = Move::Left,
            'l' => moves[3] = Move::Down,
            'i' => moves[3] = Move::Right,

            _ => {} // All players stay
        }

        moves
    }
}
