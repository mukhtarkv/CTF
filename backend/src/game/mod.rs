mod player_move;

use core::f32;

use itertools::Itertools;
pub use player_move::Move;

#[derive(Debug, Clone)]
struct Map<const N: usize> {
    player_spawn_x: [usize; N],
    player_spawn_y: [usize; N],

    flag_spawn_x: [usize; 2],
    flag_spawn_y: [usize; 2],

    wall_x: Vec<usize>,
    wall_y: Vec<usize>,

    width: usize,
    height: usize,
}

// TODO: implement dynamically loading maps
impl<const N: usize> Map<N> {
    const WIDTH: usize = 28;
    const HEIGHT: usize = 14;

    pub fn new() -> Self {
        assert!(N == 2 || N == 4, "A map must have either 2 or 4 players");
        assert!(Self::WIDTH % 2 == 0, "A map must have an even width");

        let mut player_spawn_x = [0; N];
        let mut player_spawn_y = [0; N];

        match N {
            2 => {
                player_spawn_x.copy_from_slice(&[0, Self::WIDTH - 1]);
                player_spawn_y.copy_from_slice(&[0, 0]);
            }
            4 => {
                player_spawn_x.copy_from_slice(&[0, 27, 0, Self::WIDTH - 1]);
                player_spawn_y.copy_from_slice(&[0, 0, Self::HEIGHT - 1, Self::HEIGHT - 1]);
            }
            _ => unreachable!(),
        }

        Self {
            player_spawn_x,
            player_spawn_y,
            width: Self::WIDTH,
            height: Self::HEIGHT,
            flag_spawn_x: [0, Self::WIDTH - 1],
            flag_spawn_y: [Self::HEIGHT / 2, Self::HEIGHT / 2 - 1], // -1 to make it symmetrical
            wall_x: vec![0, 27, 8, 19, 8, 9, 18, 19, 8, 9, 18, 19, 8, 19, 0, 27],
            wall_y: vec![1, 1, 2, 2, 6, 6, 6, 6, 7, 7, 7, 7, 11, 11, 12, 12],
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameState<const N: usize> {
    scores: [usize; 2],

    player_spawn_x: [f32; N],
    player_spawn_y: [f32; N],

    player_x: [f32; N],
    player_y: [f32; N],

    width: usize,
    height: usize,

    flag_spawn_x: [usize; 2],
    flag_spawn_y: [usize; 2],

    wall_x: Vec<usize>,
    wall_y: Vec<usize>,

    // Index of the player holding the flag
    // Ex. Player 3 holding the flag of team 0 -> [Some(3), None]
    flag_captors: [Option<usize>; 2],
}

impl<const N: usize> GameState<N> {
    const PLAYER_SPEED: f32 = 0.25;
    const PLAYER_SIZE: f32 = 1.0;
    const FLAG_SIZE: f32 = 1.0;
    const WALL_SIZE: f32 = 1.0;

    pub fn new() -> Self {
        assert!(N == 2 || N == 4, "A game must have either 2 or 4 players");
        let Map {
            width,
            height,
            player_spawn_x,
            player_spawn_y,
            flag_spawn_x,
            flag_spawn_y,
            wall_x,
            wall_y,
        } = Map::<N>::new();

        let player_spawn_x = player_spawn_x.map(|x| x as f32);
        let player_spawn_y = player_spawn_y.map(|y| y as f32);

        Self {
            scores: [0; 2],
            width,
            height,
            player_spawn_x: player_spawn_x,
            player_spawn_y: player_spawn_y,
            player_x: player_spawn_x,
            player_y: player_spawn_y,
            flag_spawn_x,
            flag_spawn_y,
            wall_y,
            wall_x,
            flag_captors: [None; 2],
        }
    }

    // Returns player index of the captor of team_index's flag
    fn get_flag_captor(&self, team_index: usize) -> Option<usize> {
        assert!(
            team_index == 0 || team_index == 1,
            "Team must be either 0 or 1"
        );
        self.flag_captors[team_index]
    }

    fn get_player_team(&self, player_index: usize) -> usize {
        player_index % 2
    }

    fn get_is_player_on_home_side(&self, player_index: usize, left: f32) -> bool {
        let team_index = self.get_player_team(player_index);
        let center_x_pos = left + GameState::<N>::PLAYER_SIZE / 2.0;
        let is_on_left_side = center_x_pos < (self.width as f32) / 2.0;
        (team_index == 0) && is_on_left_side || (team_index == 1) && !is_on_left_side
    }

    fn reset_player(&mut self, player_index: usize) {
        let reset_x = self.player_spawn_x[player_index];
        let reset_y = self.player_spawn_y[player_index];

        self.player_x[player_index] = reset_x;
        self.player_y[player_index] = reset_y;

        let flag_captors = self.flag_captors;
        for (team_index, flag_captor) in flag_captors.iter().enumerate() {
            if let Some(flag_captor) = flag_captor {
                if *flag_captor == player_index {
                    self.flag_captors[team_index] = None;
                }
            }
        }
    }

    pub fn step(&mut self, player_moves: [Move; N]) -> Vec<usize> {
        // Debug: Print current positions before processing
        println!("=== GAME STEP ===");
        for i in 0..N {
            println!(
                "Player {}: ({:.2}, {:.2}) Team: {}",
                i,
                self.player_x[i],
                self.player_y[i],
                self.get_player_team(i)
            );
        }
        println!("Flag captors: {:?}", self.flag_captors);

        for (player_index, player_move) in player_moves.iter().enumerate() {
            let (player_dx, player_dy) = player_move.to_coords();

            let player_x = self.player_x[player_index];
            let player_y = self.player_y[player_index];

            let speed = GameState::<N>::PLAYER_SPEED;

            // Calculate new position with bounds checking
            let new_x = (player_x + ((player_dx as f32) * speed))
                .max(0.0)
                .min((self.width as f32) - GameState::<N>::PLAYER_SIZE);
            let new_y = (player_y + ((player_dy as f32) * speed))
                .max(0.0)
                .min((self.height as f32) - GameState::<N>::PLAYER_SIZE);

            // Check if the new position would collide with walls
            if !self.would_collide_with_walls(new_x, new_y) {
                // No collision, move to new position
                self.player_x[player_index] = new_x;
                self.player_y[player_index] = new_y;
            } else {
                // Try sliding along walls
                // Try horizontal movement only
                if player_dx != 0 && !self.would_collide_with_walls(new_x, player_y) {
                    self.player_x[player_index] = new_x;
                }
                // Try vertical movement only
                if player_dy != 0 && !self.would_collide_with_walls(player_x, new_y) {
                    self.player_y[player_index] = new_y;
                }
            }
        }

        // Handle player collisions
        let players_to_reset_moves = Vec::new();
        let player_indices: [usize; N] = std::array::from_fn(|i| i);
        for pair in player_indices.iter().combinations(2) {
            let player_index_0 = *pair[0];
            let player_index_1 = *pair[1];

            if self.get_player_team(player_index_0) == self.get_player_team(player_index_1) {
                continue;
            }

            let player_0_x = self.player_x[player_index_0];
            let player_0_y = self.player_y[player_index_0];
            let player_1_x = self.player_x[player_index_1];
            let player_1_y = self.player_y[player_index_1];

            let player_0_left = player_0_x;
            let player_0_top = player_0_y;
            let player_0_right = player_0_left + GameState::<N>::PLAYER_SIZE;
            let player_0_bottom = player_0_top + GameState::<N>::PLAYER_SIZE;

            let player_1_left = player_1_x;
            let player_1_top = player_1_y;
            let player_1_right = player_1_left + GameState::<N>::PLAYER_SIZE;
            let player_1_bottom = player_1_top + GameState::<N>::PLAYER_SIZE;

            let is_collide_x = player_0_left < player_1_right && player_1_left < player_0_right;
            let is_collide_y = player_0_top < player_1_bottom && player_1_top < player_0_bottom;
            let is_collide = is_collide_x && is_collide_y;

            if !is_collide {
                continue;
            }

            // Track which players need their moves reset
            let mut players_to_reset_moves = Vec::new();

            let actual_indices = [player_index_0, player_index_1];
            for (i, left) in [player_0_left, player_1_left].iter().enumerate() {
                let player_index = actual_indices[i];
                if !self.get_is_player_on_home_side(player_index, *left) {
                    self.reset_player(player_index);
                    players_to_reset_moves.push(player_index);
                    players_to_reset_moves.push(player_index);
                }
            }

            // Return the list of players whose moves should be reset
            return players_to_reset_moves;
        }

        // No players need move reset - continue with flag captures

        // Handle flag captures
        for player_index in player_indices {
            let player_x = self.player_x[player_index];
            let player_y = self.player_y[player_index];
            let player_team = self.get_player_team(player_index);

            for team_index in [0, 1] {
                // Skip if trying to capture own team's flag
                if player_team == team_index {
                    continue;
                }

                // Skip if flag is already captured
                if self.get_flag_captor(team_index).is_some() {
                    continue;
                }

                let flag_spawn_x = self.flag_spawn_x[team_index] as f32;
                let flag_spawn_y = self.flag_spawn_y[team_index] as f32;

                // Calculate distance between player center and flag center
                let player_center_x = player_x + GameState::<N>::PLAYER_SIZE / 2.0;
                let player_center_y = player_y + GameState::<N>::PLAYER_SIZE / 2.0;
                let flag_center_x = flag_spawn_x + GameState::<N>::FLAG_SIZE / 2.0;
                let flag_center_y = flag_spawn_y + GameState::<N>::FLAG_SIZE / 2.0;

                let distance = ((player_center_x - flag_center_x).powi(2)
                    + (player_center_y - flag_center_y).powi(2))
                .sqrt();
                let capture_distance = 1.2; // Generous capture distance

                if distance <= capture_distance {
                    self.flag_captors[team_index] = Some(player_index);
                    println!(
                        "🚩 Player {} captured team {}'s flag! Distance: {:.2}",
                        player_index, team_index, distance
                    );
                }
            }
        }

        // Handle score
        let flag_captors = self.flag_captors;
        for (team_index, flag_captor) in flag_captors.iter().enumerate() {
            if let Some(player_index) = flag_captor {
                let left = self.player_x[*player_index];
                if self.get_is_player_on_home_side(*player_index, left) {
                    self.flag_captors[team_index] = None;
                    let scoring_team_index = self.get_player_team(*player_index);
                    self.scores[scoring_team_index] += 1;
                    println!(
                        "🎯 SCORE! Player {} scored for team {}! New scores: {:?}",
                        player_index, scoring_team_index, self.scores
                    );

                    // Reset all players to spawn positions after a score
                    for i in 0..N {
                        self.player_x[i] = self.player_spawn_x[i];
                        self.player_y[i] = self.player_spawn_y[i];
                    }
                    println!("🔄 All players reset to spawn positions after score!");
                }
            }
        }

        println!("Final flag captors: {:?}", self.flag_captors);
        println!("=================");

        // Return list of players whose moves should be reset
        players_to_reset_moves
    }

    fn would_collide_with_walls(&self, x: f32, y: f32) -> bool {
        let player_left = x;
        let player_top = y;
        let player_right = x + GameState::<N>::PLAYER_SIZE;
        let player_bottom = y + GameState::<N>::PLAYER_SIZE;

        for (wall_x, wall_y) in self.wall_x.iter().zip(self.wall_y.iter()) {
            let wall_left = *wall_x as f32;
            let wall_top = *wall_y as f32;
            let wall_right = wall_left + GameState::<N>::WALL_SIZE;
            let wall_bottom = wall_top + GameState::<N>::WALL_SIZE;

            let is_collide_x = player_left < wall_right && wall_left < player_right;
            let is_collide_y = player_top < wall_bottom && wall_top < player_bottom;

            if is_collide_x && is_collide_y {
                return true;
            }
        }
        false
    }

    pub fn positions(&self) -> Vec<(f32, f32)> {
        (0..N)
            .map(|i| (self.player_x[i], self.player_y[i]))
            .collect()
    }

    pub fn get_flag_captors(&self) -> [Option<usize>; 2] {
        self.flag_captors
    }

    pub fn get_scores(&self) -> [usize; 2] {
        self.scores
    }

    pub fn pretty_print(&self) {
        print!(
            "Blue {:>2} {:->15} Red {:>2}\r\n",
            self.scores[0], "", self.scores[1]
        );

        let mut grid: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        print!("┌{:->width$}┐\r\n", "", width = self.width);

        // Print players
        let player_x_iter = self.player_x.iter();
        let player_y_iter = self.player_y.iter();
        for (i, (x, y)) in player_x_iter.zip(player_y_iter).enumerate() {
            let x_rounded = x.floor() as usize;
            let y_rounded = y.floor() as usize;
            grid[y_rounded][x_rounded] = i.to_string().chars().next().unwrap_or('_');
        }

        // Print walls
        let wall_x_iter = self.wall_x.iter();
        let wall_y_iter = self.wall_y.iter();
        for (x, y) in wall_x_iter.zip(wall_y_iter) {
            grid[*y][*x] = '█';
        }

        // Print flags
        for team_index in [0, 1] {
            let flag_char = match team_index {
                0 => 'B',
                1 => 'R',
                _ => unreachable!("Team index must be 0 or 1"),
            };
            let flag_captor = self.get_flag_captor(team_index);
            match flag_captor {
                // TODO: draw when it's been captured
                Some(_player_index) => (),
                None => {
                    let flag_x = self.flag_spawn_x[team_index];
                    let flag_y = self.flag_spawn_y[team_index];

                    grid[flag_y][flag_x] = flag_char;
                }
            }
        }

        grid.iter().for_each(|row| {
            let string: String = row.iter().collect();
            print!("|{}|\r\n", string);
        });

        print!("└{:->width$}┘\r\n", "", width = self.width);
    }
}

#[cfg(test)]
mod dev_test {
    use super::*;

    #[test]
    fn quick_dev() {
        let mut game = GameState::<4>::new();
        game.pretty_print();
        for _ in (0..104) {
            game.step([Move::Right, Move::Stay, Move::Stay, Move::Stay]);
        }
        for _ in (0..24) {
            game.step([Move::Down, Move::Stay, Move::Stay, Move::Stay]);
        }
        for _ in (0..4) {
            game.step([Move::Right, Move::Stay, Move::Stay, Move::Stay]);
        }
        for _ in (0..4) {
            game.step([Move::Up, Move::Stay, Move::Stay, Move::Stay]);
        }
        for _ in (0..96) {
            game.step([Move::Left, Move::Stay, Move::Stay, Move::Stay]);
        }
        game.pretty_print();
    }
}
