use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Free,
    Occupied,
    Floor,
}

impl State {
    fn is_occupied(&self) -> bool {
        matches!(self, State::Occupied)
    }
    fn is_free(&self) -> bool {
        matches!(self, State::Free)
    }
    fn is_floor(&self) -> bool {
        matches!(self, State::Floor)
    }
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Free,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

struct Grid {
    grid: Vec<Vec<State>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        Self {
            grid: input
                .lines()
                .map(|s| s.chars().map(Into::into).collect())
                .collect(),
        }
    }

    fn count_occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .flatten()
            .filter(|x| x.is_occupied())
            .count()
    }

    fn occupy(&mut self, r: usize, c: usize) {
        self.grid[r][c] = State::Occupied;
    }

    fn free(&mut self, r: usize, c: usize) {
        self.grid[r][c] = State::Free;
    }

    fn window1(&self) -> impl Iterator<Item = ((usize, usize), Vec<State>)> + '_ {
        let nrows = self.grid.len() as i32;
        let ncols = self.grid[0].len() as i32;

        let mut row = 0;
        let mut col = -1;

        std::iter::from_fn(move || {
            row += (col + 1 == ncols) as i32;
            col = (col + 1) % ncols;

            if row < nrows {
                let window: Vec<State> = (-1..=1)
                    .flat_map(|delta_row| {
                        (-1..=1).map(move |delta_col| (row + delta_row, col + delta_col))
                    })
                    .filter(|(r, c)| {
                        !(*r == row && *c == col) && *r >= 0 && *r < nrows && *c >= 0 && *c < ncols
                    })
                    .map(|(r, c)| self.grid[r as usize][c as usize])
                    .filter(|x| !x.is_floor())
                    .collect();

                return Some(((row as usize, col as usize), window));
            }

            None
        })
    }

    fn window_fn(
        &self,
        f: fn(State) -> bool,
    ) -> impl Iterator<Item = ((usize, usize), Vec<State>)> + '_ {
        let nrows = self.grid.len() as i32;
        let ncols = self.grid[0].len() as i32;

        let mut row = 0;
        let mut col = -1;

        std::iter::from_fn(move || {
            row += (col + 1 == ncols) as i32;
            col = (col + 1) % ncols;

            let valid_seat = |r: i32, c: i32| {
                !(r == row && c == col) && r >= 0 && r < nrows && c >= 0 && c < ncols
            };

            if row < nrows {
                let window: Vec<State> = (-1..=1)
                    .flat_map(|delta_row| {
                        (-1..=1).filter_map(move |delta_col| {
                            let mut rr = row + delta_row;
                            let mut cc = col + delta_col;

                            while valid_seat(rr, cc) && f(self.grid[rr as usize][cc as usize]) {
                                rr += delta_row;
                                cc += delta_col;
                            }

                            if valid_seat(rr, cc) && !f(self.grid[rr as usize][cc as usize]) {
                                Some((rr, cc))
                            } else {
                                None
                            }
                        })
                    })
                    .map(|(r, c)| self.grid[r as usize][c as usize])
                    .collect();

                return Some(((row as usize, col as usize), window));
            }

            None
        })
    }
}

fn main() {
    println!("{}", simulate_arrivals1(Grid::new(include_str!("input"))));
    println!("{}", simulate_arrivals2(Grid::new(include_str!("input"))));
}

fn simulate_arrivals1(mut seats: Grid) -> usize {
    let mut changes = 1;
    let mut to_occupy = HashSet::new();
    let mut to_free = HashSet::new();

    while changes != 0 {
        changes = 0;

        for ((i, j), window) in seats.window1() {
            let occupied = window.iter().filter(|x| x.is_occupied()).count();

            if occupied >= 4 && seats.grid[i][j].is_occupied() {
                to_free.insert((i, j));
                changes += 1;
            } else if occupied == 0 && seats.grid[i][j].is_free() {
                to_occupy.insert((i, j));
                changes += 1;
            }
        }

        to_free.iter().for_each(|(i, j)| seats.free(*i, *j));
        to_free.clear();

        to_occupy.iter().for_each(|(i, j)| seats.occupy(*i, *j));
        to_occupy.clear();
    }

    seats.count_occupied_seats()
}

fn simulate_arrivals2(mut seats: Grid) -> usize {
    let mut changes = 1;
    let mut to_occupy = HashSet::new();
    let mut to_free = HashSet::new();

    while changes != 0 {
        changes = 0;

        for ((i, j), window) in seats.window_fn(|x| x.is_floor()) {
            let occupied = window.iter().filter(|x| x.is_occupied()).count();

            if occupied >= 5 && seats.grid[i][j].is_occupied() {
                to_free.insert((i, j));
                changes += 1;
            } else if occupied == 0 && seats.grid[i][j].is_free() {
                to_occupy.insert((i, j));
                changes += 1;
            }
        }

        to_free.iter().for_each(|(i, j)| seats.free(*i, *j));
        to_free.clear();

        to_occupy.iter().for_each(|(i, j)| seats.occupy(*i, *j));
        to_occupy.clear();
    }

    seats.count_occupied_seats()
}
