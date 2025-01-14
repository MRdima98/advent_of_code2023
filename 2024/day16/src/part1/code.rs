use core::time;
use std::{
    collections::HashMap,
    fmt::Display,
    fs,
    ops::{Add, AddAssign, Sub, SubAssign},
    thread, usize,
};

pub fn run(path: &str) {
    let input: String = fs::read_to_string(path).unwrap();
    let mut map = vec![];
    let mut start = Coord(0, 0);
    let mut goal = Coord(0, 0);

    for (i, line) in input.lines().enumerate() {
        let mut tmp = vec![];
        for (j, el) in line.chars().enumerate() {
            tmp.push(el);
            if el == 'S' {
                start = Coord(i, j);
            }

            if el == 'E' {
                goal = Coord(i, j);
            }
        }
        map.push(tmp);
    }

    let path = a_star(&map, start, goal);
}

fn reconstruct_path(came_from: HashMap<Coord, Coord>, current: Coord) -> Vec<Coord> {
    let mut total_path = vec![current];

    let mut current = current;
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        total_path.push(current);
    }

    total_path
}

fn heuritis(neighbour: Coord, goal: Coord) -> usize {
    (((neighbour.0 as i64 - goal.0 as i64).pow(2) + (neighbour.1 as i64 - goal.1 as i64).pow(2))
        .abs() as f64)
        .sqrt() as usize
}

fn a_star(map: &[Vec<char>], start: Coord, goal: Coord) -> Vec<Coord> {
    let mut open_set = vec![start];
    let mut came_from: HashMap<Coord, Coord> = HashMap::new();

    let mut g_score: HashMap<Coord, (usize, Direction)> = HashMap::new();
    g_score.insert(start, (0, Direction::Left));

    let mut f_score: HashMap<Coord, usize> = HashMap::new();
    f_score.insert(start, heuritis(start, goal));

    while !open_set.is_empty() {
        let mut min = usize::MAX;
        let mut coord = Coord(0, 0);
        for node in open_set.iter() {
            if let Some(val) = f_score.get(node) {
                if *val < min {
                    coord = *node;
                    min = *val;
                }
            }
        }

        let mut idx = 0;
        for (i, el) in open_set.iter().enumerate() {
            if coord == *el {
                idx = i;
            }
        }

        let current = open_set.remove(idx);

        if current == goal {
            return reconstruct_path(came_from, current);
        }

        let neighbours = get_neighbours(map, current);

        for neigh in neighbours {
            let current_score = g_score.get(&current).unwrap();
            let mut current_direction = current_score.1;
            let tentative_score = current_score.0 + dist(current, neigh, &mut current_direction);

            g_score
                .entry(neigh)
                .or_insert((usize::max_value(), current_direction));

            f_score.entry(neigh).or_insert(usize::max_value());

            if tentative_score < g_score.get(&neigh).unwrap().0 {
                came_from.entry(neigh).or_insert(current);

                g_score
                    .entry(neigh)
                    .and_modify(|el| *el = (tentative_score, current_direction));

                f_score
                    .entry(neigh)
                    .and_modify(|el| *el = tentative_score + heuritis(neigh, goal));

                if !open_set.contains(&neigh) {
                    open_set.push(neigh);
                }
            }
        }
    }

    vec![]
}

fn dist(current: Coord, neigh: Coord, dir: &mut Direction) -> usize {
    let mut dist = 0;
    let mut actual_dir: Option<Direction> = None;
    let next_move = (
        current.0 as i64 - neigh.0 as i64,
        current.1 as i64 - neigh.1 as i64,
    );

    match next_move {
        (-1, 0) => {
            if matches!(dir, Direction::Up) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Up);
            }
        }
        (1, 0) => {
            if matches!(dir, Direction::Down) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Down);
            }
        }
        (0, 1) => {
            if matches!(dir, Direction::Right) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Right);
            }
        }
        (0, -1) => {
            if matches!(dir, Direction::Left) {
                dist += 1;
            } else {
                actual_dir = Some(Direction::Left);
            }
        }
        (_, _) => {}
    }

    if dist == 1 {
        return 1;
    }

    if let Some(actual_dir) = actual_dir {
        let clockwise = rotate(&mut dir.clone(), actual_dir.clone(), "CLOCKWISE");
        let anti_clockwise = rotate(&mut dir.clone(), actual_dir, "ANTI-CLOCKWISE");
        *dir = actual_dir;
        return clockwise.min(anti_clockwise);
    }

    dist
}

fn rotate(dir: &mut Direction, actual_dir: Direction, which_way: &str) -> usize {
    let mut counter = 0;
    let clockwise = "CLOCKWISE";
    loop {
        if *dir == actual_dir {
            break;
        }

        match dir {
            Direction::Up => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Right;
                } else {
                    *dir = Direction::Left;
                }
            }
            Direction::Down => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Left;
                } else {
                    *dir = Direction::Right;
                }
            }
            Direction::Left => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Up;
                } else {
                    *dir = Direction::Down;
                }
            }
            Direction::Right => {
                counter += 1000;
                if which_way == clockwise {
                    *dir = Direction::Down;
                } else {
                    *dir = Direction::Up;
                }
            }
        }
    }

    counter + 1
}

fn get_neighbours(map: &[Vec<char>], current: Coord) -> Vec<Coord> {
    let mut possible_neighbours = vec![];
    possible_neighbours.push(current - Coord(1, 0));
    possible_neighbours.push(current + Coord(1, 0));
    possible_neighbours.push(current - Coord(0, 1));
    possible_neighbours.push(current + Coord(0, 1));

    let mut neighbours = vec![];
    for neigh in possible_neighbours {
        if map[neigh.0][neigh.1] == '.' || map[neigh.0][neigh.1] == 'E' {
            neighbours.push(neigh);
        }
    }

    neighbours
}

#[derive(Clone, Debug, Copy, Eq, Hash, PartialEq, PartialOrd)]
struct Coord(usize, usize);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn pretty_print(map: &[Vec<char>]) {
    for line in map {
        for el in line {
            print!("{el}");
        }
        println!();
    }
    println!();
}

fn print_surr(map: &[Vec<char>], coord: Coord) {
    for (i, line) in map.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            if i == coord.0 && j == coord.1 {
                print!("X");
            } else {
                print!("{el}");
            }
        }
        println!();
    }
    println!();
}
