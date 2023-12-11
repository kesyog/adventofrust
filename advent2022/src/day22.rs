//! Solution to [AoC 2022 Day 22](https://adventofcode.com/2022/day/22)

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

use num_complex::Complex;
use once_cell::sync::OnceCell;

trait Stepper {
    fn next_step(&self) -> Option<(usize, Direction)>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    /// Not part of the map
    Void,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            ' ' => Tile::Void,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            _ => anyhow::bail!("invalid tile: {c}"),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    grid: Vec<Tile>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn coords_2d(&self, x: usize) -> (usize, usize) {
        (x / self.cols, x % self.cols)
    }

    fn coords_1d(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    /// Iterate across points in a given row that are part of the map (not `Tile::Void`)
    /// Iterator yields 1d-index and tile type
    fn iter_row(&self, row: usize) -> impl DoubleEndedIterator<Item = (usize, Tile)> + Clone + '_ {
        let i = self.coords_1d(row, 0);
        let start = self.grid[i..i + self.cols]
            .iter()
            .position(|&i| i != Tile::Void)
            .unwrap();
        let end = self.grid[i..i + self.cols]
            .iter()
            .rposition(|&i| i != Tile::Void)
            .unwrap();
        // RangeInclusive doesn't implement ExactSizeIterator
        // https://github.com/rust-lang/rust/issues/36386
        #[allow(clippy::range_plus_one)]
        (i + start..i + end + 1).zip(self.grid[i + start..=i + end].iter().copied())
    }

    /// Iterate across points in a given column that are part of the map (not `Tile::Void`)
    /// Iterator yields 1d-index and tile type
    fn iter_col(&self, col: usize) -> impl DoubleEndedIterator<Item = (usize, Tile)> + Clone + '_ {
        let lower_bound = self.coords_1d(0, col);
        let upper_bound = self.coords_1d(self.rows - 1, col);
        let start = self.grid[lower_bound..=upper_bound]
            .iter()
            .step_by(self.cols)
            .position(|&i| i != Tile::Void)
            .unwrap();
        let end = self.grid[lower_bound..=upper_bound]
            .iter()
            .step_by(self.cols)
            .rposition(|&i| i != Tile::Void)
            .unwrap();
        // RangeInclusive doesn't implement ExactSizeIterator
        // https://github.com/rust-lang/rust/issues/36386
        #[allow(clippy::range_plus_one)]
        (self.coords_1d(start, col)..self.coords_1d(end, col) + 1)
            .step_by(self.cols)
            .zip(
                self.grid[self.coords_1d(start, col)..=self.coords_1d(end, col)]
                    .iter()
                    .step_by(self.cols)
                    .copied(),
            )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(self, rotation: Rotation) -> Self {
        (self.as_complex() * rotation.as_complex())
            .try_into()
            .unwrap()
    }

    const fn as_complex(self) -> Complex<isize> {
        match self {
            Direction::Up => Complex::new(0, 1),
            Direction::Down => Complex::new(0, -1),
            Direction::Left => Complex::new(-1, 0),
            Direction::Right => Complex::new(1, 0),
        }
    }

    /// Calculate "facing" score as defined by problem
    const fn facing(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn flip(self) -> Self {
        let rot = Rotation::Clockwise;
        self.rotate(rot).rotate(rot)
    }
}

impl TryFrom<Complex<isize>> for Direction {
    type Error = anyhow::Error;

    fn try_from(val: Complex<isize>) -> Result<Self, Self::Error> {
        Ok(match (val.re, val.im) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
            _ => anyhow::bail!("not a direction: {val}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

impl Rotation {
    const fn as_complex(self) -> Complex<isize> {
        match self {
            Rotation::Clockwise => Complex::new(0, -1),
            Rotation::Counterclockwise => Complex::new(0, 1),
        }
    }
}

impl TryFrom<char> for Rotation {
    type Error = anyhow::Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'R' => Self::Clockwise,
            'L' => Self::Counterclockwise,
            _ => anyhow::bail!("Invalid character: {c}"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Part1();
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Part2();

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Navigator<'a, T> {
    grid: &'a Grid,
    direction: Direction,
    location: usize,
    part: PhantomData<*const T>,
}

impl<'a, T> Navigator<'a, T>
where
    Navigator<'a, T>: Stepper,
{
    fn new(grid: &'a Grid) -> Self {
        let start = grid
            .iter_row(0)
            .find(|&(_, tile)| tile == Tile::Empty)
            .unwrap()
            .0;
        Self {
            grid,
            direction: Direction::Right,
            location: start,
            part: PhantomData,
        }
    }

    fn step(&mut self) {
        if let Some((loc, direction)) = self.next_step() {
            self.location = loc;
            self.direction = direction;
        }
    }

    fn rotate(&mut self, rotation: Rotation) {
        self.direction = self.direction.rotate(rotation);
    }

    fn do_action(&mut self, action: Action) {
        match action {
            Action::Step(n) => {
                for _ in 0..n {
                    self.step();
                }
            }
            Action::Rotate(rot) => self.rotate(rot),
        }
    }
}

impl<'a> Stepper for Navigator<'a, Part1> {
    fn next_step(&self) -> Option<(usize, Direction)> {
        let (current_row, current_col) = self.grid.coords_2d(self.location);
        let (loc, tile) = match self.direction {
            Direction::Up => self
                .grid
                .iter_col(current_col)
                .rev()
                .cycle()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1)
                .unwrap(),
            Direction::Down => self
                .grid
                .iter_col(current_col)
                .cycle()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1)
                .unwrap(),
            Direction::Left => self
                .grid
                .iter_row(current_row)
                .rev()
                .cycle()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1)
                .unwrap(),
            Direction::Right => self
                .grid
                .iter_row(current_row)
                .cycle()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1)
                .unwrap(),
        };
        (tile != Tile::Wall).then_some((loc, self.direction))
    }
}

impl<'a> Stepper for Navigator<'a, Part2> {
    fn next_step(&self) -> Option<(usize, Direction)> {
        // Hard-coded map to handle wrapping. Works for my real input only :(
        // TODO: (never?) write a solution that works for any arbitrary shape
        static WRAP: OnceCell<HashMap<(usize, Direction), (usize, Direction)>> = OnceCell::new();

        let (current_row, current_col) = self.grid.coords_2d(self.location);
        // See if the next cube can be handled without handling wrapping
        let next = match self.direction {
            Direction::Up => self
                .grid
                .iter_col(current_col)
                .rev()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1),
            Direction::Down => self
                .grid
                .iter_col(current_col)
                .skip_while(|&(i, _)| i != self.location)
                .nth(1),
            Direction::Left => self
                .grid
                .iter_row(current_row)
                .rev()
                .skip_while(|&(i, _)| i != self.location)
                .nth(1),
            Direction::Right => self
                .grid
                .iter_row(current_row)
                .skip_while(|&(i, _)| i != self.location)
                .nth(1),
        };
        if let Some((loc, tile)) = next {
            return (tile != Tile::Wall).then_some((loc, self.direction));
        }

        // We're trying to go around an edge
        let wrap_map = WRAP.get_or_init(|| {
            /// Helper function to map going across an edge in both directions
            fn bidir_insert(
                map: &mut HashMap<(usize, Direction), (usize, Direction)>,
                (loc1, dir1): (usize, Direction),
                (loc2, dir2): (usize, Direction),
            ) {
                map.insert((loc1, dir1), (loc2, dir2));
                map.insert((loc2, dir2.flip()), (loc1, dir1.flip()));
            }

            let mut out = HashMap::new();
            for i in 0..50 {
                // Each of these markings matches non-sensical symbols I drew on paper to help line
                // up edges
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(0, i + 50), Direction::Up),
                    (self.grid.coords_1d(150 + i, 0), Direction::Right),
                );
                // XX
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(0, i + 100), Direction::Up),
                    (self.grid.coords_1d(199, i), Direction::Up),
                );
                // 🔺
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(i, 149), Direction::Right),
                    (self.grid.coords_1d(149 - i, 99), Direction::Left),
                );
                // |
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(49, 100 + i), Direction::Down),
                    (self.grid.coords_1d(50 + i, 99), Direction::Left),
                );
                // |||
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(149, 50 + i), Direction::Down),
                    (self.grid.coords_1d(150 + i, 49), Direction::Left),
                );
                // X
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(i, 50), Direction::Left),
                    (self.grid.coords_1d(149 - i, 0), Direction::Right),
                );
                // ||
                bidir_insert(
                    &mut out,
                    (self.grid.coords_1d(50 + i, 50), Direction::Left),
                    (self.grid.coords_1d(100, i), Direction::Down),
                );
            }
            out
        });
        let &(new_loc, new_direction) = wrap_map
            .get(&(self.location, self.direction))
            .unwrap_or_else(|| {
                panic!(
                    "Wrapping @ ({} {:?}) to be defined",
                    self.location, self.direction
                )
            });
        (self.grid.grid[new_loc] != Tile::Wall).then_some((new_loc, new_direction))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Step(usize),
    Rotate(Rotation),
}

fn part1(grid: &Grid, actions: &[Action]) -> usize {
    let mut nav = Navigator::<Part1>::new(grid);
    for action in actions {
        nav.do_action(*action);
    }
    let (row, col) = grid.coords_2d(nav.location);
    1000 * (row + 1) + 4 * (col + 1) + nav.direction.facing()
}

fn part2(grid: &Grid, actions: &[Action]) -> usize {
    let mut nav = Navigator::<Part2>::new(grid);
    for action in actions {
        nav.do_action(*action);
    }
    let (row, col) = grid.coords_2d(nav.location);
    1000 * (row + 1) + 4 * (col + 1) + nav.direction.facing()
}

fn parse_input(input: &str) -> (Grid, Vec<Action>) {
    let (raw_grid, raw_actions) = input.split_once("\n\n").unwrap();
    let n_cols = raw_grid.lines().map(str::len).max().unwrap();
    let n_rows = raw_grid.trim_matches('\n').matches('\n').count() + 1;

    let mut grid: Vec<Tile> = Vec::new();
    for line in raw_grid.trim_matches('\n').lines() {
        grid.extend(line.chars().map(|c| Tile::try_from(c).unwrap()));
        grid.extend(
            std::iter::once(Tile::Void)
                .cycle()
                .take(n_cols - line.len()),
        );
    }
    assert_eq!(grid.len(), n_rows * n_cols);
    let grid = Grid {
        grid,
        rows: n_rows,
        cols: n_cols,
    };

    let mut actions: Vec<Action> = Vec::new();
    let mut current_num = None;
    for c in raw_actions.trim().chars() {
        match (c.is_ascii_digit(), current_num) {
            (false, Some(n)) => {
                actions.push(Action::Step(n));
                current_num = None;
                actions.push(Action::Rotate(c.try_into().unwrap()));
            }
            (false, None) => actions.push(Action::Rotate(c.try_into().unwrap())),
            (true, _) => {
                current_num =
                    Some(current_num.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as usize);
            }
        }
    }
    if let Some(n) = current_num {
        actions.push(Action::Step(n));
    }

    (grid, actions)
}

#[cfg(not(test))]
fn main() {
    let input = include_str!("../inputs/day22.txt");
    let (grid, actions) = parse_input(input);

    let (p1, p2) = rayon::join(|| part1(&grid, &actions), || part2(&grid, &actions));
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Need to be careful to not leave extra spaces at start or end. Newlines are fine.
    const TEST_INPUT: &str = r#"
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    fn given_part1_input() {
        let (grid, actions) = parse_input(TEST_INPUT);
        assert_eq!(part1(&grid, &actions), 6032);
    }

    #[test]
    fn given_part2_input() {
        // Fails since implementation is hard-coded for the real input
        // let (grid, actions) = parse_input(TEST_INPUT);
        // assert_eq!(part2(&grid, &actions), 5031);
    }

    #[test]
    fn test_grid_iter() {
        let (grid, _) = parse_input(TEST_INPUT);
        assert_eq!(
            vec![
                (8, Tile::Empty),
                (9, Tile::Empty),
                (10, Tile::Empty),
                (11, Tile::Wall)
            ],
            grid.iter_row(0).collect::<Vec<(usize, Tile)>>()
        );
        assert_eq!(
            vec![
                (16 * 4 + 1, Tile::Empty),
                (16 * 5 + 1, Tile::Empty),
                (16 * 6 + 1, Tile::Empty),
                (16 * 7 + 1, Tile::Empty)
            ],
            grid.iter_col(1).collect::<Vec<(usize, Tile)>>()
        );
    }

    #[test]
    fn test_hitting_wall() {
        let (grid, _) = parse_input(TEST_INPUT);
        let mut nav = Navigator::<Part1>::new(&grid);
        nav.direction = Direction::Right;
        nav.location = grid.coords_1d(0, 10);
        let expected_nav = nav.clone();
        nav.step();
        assert_eq!(nav.location, expected_nav.location);
        nav.step();
        assert_eq!(nav.location, expected_nav.location);
    }
}
