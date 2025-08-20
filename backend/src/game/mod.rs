mod player_move;

use core::f32;

use itertools::Itertools;
use player_move::Move;

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
struct GameState<const N: usize> {
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
    }

    pub fn step(&mut self, player_moves: [Move; N]) {
        for (player_index, player_move) in player_moves.iter().enumerate() {
            let (player_dx, player_dy) = player_move.to_coords();

            let player_x = self.player_x[player_index];
            let player_y = self.player_y[player_index];

            let speed = GameState::<N>::PLAYER_SPEED;

            // Potentially illegal states
            let mut new_player_x = player_x + ((player_dx as f32) * speed);
            let mut new_player_y = player_y + ((player_dy as f32) * speed);

            // Handle bounds
            let is_out_top = new_player_y < 0.0;
            let is_out_left = new_player_x < 0.0;
            let is_out_right = (new_player_x + GameState::<N>::PLAYER_SIZE) > (self.width as f32);
            let is_out_bottom = (new_player_y + GameState::<N>::PLAYER_SIZE) > (self.height as f32);

            if is_out_top {
                new_player_y = 0.0;
            }
            if is_out_left {
                new_player_x = 0.0;
            }
            if is_out_right {
                new_player_x = (self.width as f32) - GameState::<N>::PLAYER_SIZE;
            }
            if is_out_bottom {
                new_player_y = (self.height as f32) - GameState::<N>::PLAYER_SIZE;
            }

            // Handle wall collisions
            let player_left = player_x as f32;
            let player_top = player_y as f32;
            let player_right = player_x + GameState::<N>::PLAYER_SIZE;
            let player_bottom = player_y + GameState::<N>::PLAYER_SIZE;

            let wall_x_iter = self.wall_x.iter();
            let wall_y_iter = self.wall_y.iter();
            for (x, y) in wall_x_iter.zip(wall_y_iter) {
                let wall_left = *x as f32;
                let wall_top = *y as f32;
                let wall_right = wall_left + GameState::<N>::WALL_SIZE;
                let wall_bottom = wall_top + GameState::<N>::WALL_SIZE;

                let is_collide_x = player_left < wall_right && wall_left < player_right;
                let is_collide_y = player_top < wall_bottom && wall_top < player_bottom;
                let is_collide = is_collide_x && is_collide_y;

                if !is_collide {
                    continue;
                }

                // Overlaps are relative to the four sides of the wall
                let top_overlap = player_bottom - wall_top;
                let bottom_overlap = wall_bottom - player_top;
                let left_overlap = player_right - wall_left;
                let right_overlap = wall_right - player_left;

                // Minimum overlap is the one that needs to be adjusted
                let min_overlap = [top_overlap, bottom_overlap, left_overlap, right_overlap]
                    .iter()
                    .fold(f32::INFINITY, |acc, &x| acc.min(x));

                if min_overlap == top_overlap {
                    new_player_y = wall_top - GameState::<N>::PLAYER_SIZE;
                }
                if min_overlap == bottom_overlap {
                    new_player_y = wall_bottom;
                }
                if min_overlap == left_overlap {
                    new_player_x = wall_left - GameState::<N>::PLAYER_SIZE;
                }
                if min_overlap == right_overlap {
                    new_player_x = wall_right;
                }
            }

            self.player_x[player_index] = new_player_x;
            self.player_y[player_index] = new_player_y;
        }

        // Handle player collisions
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

            for (player_index, left) in [player_0_left, player_1_left].iter().enumerate() {
                if !self.get_is_player_on_home_side(player_index, *left) {
                    self.reset_player(player_index);
                }
            }

            println!(
                "Collision between {} and {}",
                player_index_0, player_index_1
            );
        }

        // Handle flag captures
        for player_index in player_indices {
            for team_index in [0, 1] {
                let player_team = self.get_player_team(player_index);
                let is_flag_captured = self.get_flag_captor(team_index).is_some();
                if player_team == team_index {
                    // Cannot capture own flag
                    continue;
                }
                if is_flag_captured {
                    // Unavailable for capture
                    continue;
                }

                let player_x = self.player_x[player_index];
                let player_y = self.player_y[player_index];

                let flag_spawn_x = self.flag_spawn_x[team_index] as f32;
                let flag_spawn_y = self.flag_spawn_y[team_index] as f32;

                let player_left = player_x;
                let player_top = player_y;
                let player_right = player_left + GameState::<N>::PLAYER_SIZE;
                let player_bottom = player_top + GameState::<N>::PLAYER_SIZE;

                let flag_left = flag_spawn_x;
                let flag_top = flag_spawn_y;
                let flag_right = flag_left + GameState::<N>::FLAG_SIZE;
                let flag_bottom = flag_top + GameState::<N>::FLAG_SIZE;

                let is_collide_x = player_left < flag_right && flag_left < player_right;
                let is_collide_y = player_top < flag_bottom && flag_top < player_bottom;
                let is_collide = is_collide_x && is_collide_y;

                if !is_collide {
                    continue;
                }

                self.flag_captors[team_index] = Some(player_index);
                println!(
                    "Player {} is capturing team {}'s flag",
                    player_index, team_index
                );
            }
        }

        // Handle score
        let flag_captors = self.flag_captors;
        for (team_index, flag_captor) in flag_captors.iter().enumerate() {
            if let Some(player_index) = flag_captor {
                let left = self.player_x[*player_index];
                if self.get_is_player_on_home_side(*player_index, left) {
                    self.flag_captors[team_index] = None;
                    println!("Score for team {}!", self.get_player_team(*player_index));
                }
            }
        }
    }

    pub fn pretty_print(&self) {
        println!(
            "Blue {:>2} {:->15} Red {:>2}",
            self.scores[0], "", self.scores[1]
        );

        let mut grid: Vec<Vec<char>> = vec![vec![' '; self.width]; self.height];

        println!("┌{:->width$}┐", "", width = self.width);

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
                Some(player_index) => (),
                None => {
                    let flag_x = self.flag_spawn_x[team_index];
                    let flag_y = self.flag_spawn_y[team_index];

                    grid[flag_y][flag_x] = flag_char;
                }
            }
        }

        grid.iter().for_each(|row| {
            let string: String = row.iter().collect();
            println!("|{}|", string);
        });

        println!("└{:->width$}┘", "", width = self.width);
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
