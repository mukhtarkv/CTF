#[derive(Debug, Clone)]
struct Map<const N: usize> {
    player_x: [usize; N],
    player_y: [usize; N],

    flag_spawn_x: [usize; 2],
    flag_spawn_y: [usize; 2],

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

        let mut player_x = [0; N];
        let mut player_y = [0; N];

        match N {
            2 => {
                player_x.copy_from_slice(&[0, Self::WIDTH - 1]);
                player_y.copy_from_slice(&[0, 0]);
            }
            4 => {
                player_x.copy_from_slice(&[0, 27, 0, Self::WIDTH - 1]);
                player_y.copy_from_slice(&[0, 0, Self::HEIGHT - 1, Self::HEIGHT - 1]);
            }
            _ => unreachable!(),
        }

        Self {
            player_x,
            player_y,
            width: Self::WIDTH,
            height: Self::HEIGHT,
            flag_spawn_x: [0, Self::WIDTH],
            flag_spawn_y: [Self::HEIGHT / 2, Self::HEIGHT / 2],
        }
    }
}

#[derive(Debug, Clone)]
struct GameState<const N: usize> {
    scores: [usize; 2],

    player_x: [f32; N],
    player_y: [f32; N],

    width: usize,
    height: usize,

    flag_spawn_x: [usize; 2],
    flag_spawn_y: [usize; 2],

    // Index of the player holding the flag
    // Ex. Player 3 holding the flag of team 0 -> [Some(3), None]
    flag_captors: [Option<usize>; 2],
}

impl<const N: usize> GameState<N> {
    pub fn new() -> Self {
        assert!(N == 2 || N == 4, "A game must have either 2 or 4 players");
        let Map {
            width,
            height,
            player_x,
            player_y,
            flag_spawn_x,
            flag_spawn_y,
        } = Map::<N>::new();

        Self {
            scores: [0; 2],
            width,
            height,
            player_x: player_x.map(|x| x as f32),
            player_y: player_y.map(|y| y as f32),
            flag_spawn_x,
            flag_spawn_y,
            flag_captors: [None; 2],
        }
    }

    fn get_flag_captor(self, team_index: usize) -> Option<usize> {
        assert!(
            team_index == 0 || team_index == 1,
            "Team must be either 0 or 1"
        );
        self.flag_captors.get(team_index).flatten()
    }

    pub fn pretty_print(self) {
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
        let game = GameState::<4>::new();
        game.pretty_print();
    }
}
