use core::{panic, time};
use std::{collections::HashMap, fmt::Display, thread, time::SystemTime, usize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Going {
    pos: Position,
    dir: Direction,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Right => write!(f, "Right"),
            Direction::Left => write!(f, "Left"),
        }
    }
}

enum Block {
    Valid,
    Obstacle,
    Exit,
}

pub fn run() {
    let input = include_str!("../input");
    let mut map = vec![];
    let mut row: Vec<char>;
    let mut guard_coord = Position { x: 0, y: 0 };

    for (i, line) in input.lines().enumerate() {
        if let Some(guard) = line.find("^") {
            guard_coord = Position { x: i, y: guard };
        }
        row = line.chars().collect();
        map.push(row);
    }

    let mut direction = Direction::Up;
    let mut block = Block::Valid;
    let mut count = 0;
    let mut distinct_pos: Vec<Going> = vec![];

    loop {
        match block {
            Block::Exit => {
                break;
            }
            Block::Obstacle => {
                direction = check_direction(direction);
                move_guard(&mut guard_coord, &direction, &map);
            }
            _ => {}
        }

        distinct_pos.push(Going {
            pos: guard_coord,
            dir: direction,
        });

        block = move_guard(&mut guard_coord, &direction, &map);
    }

    println!("{:?}", distinct_pos.iter().count());

    count = 0;

    for (i, g) in distinct_pos.iter_mut().enumerate() {
        let mut fake_map = map.clone();
        guard_coord = g.pos.clone();
        block = move_guard(&mut g.pos, &g.dir.clone(), &map);
        //println!("Guards start: {}", guard_coord);
        //println!("Block: {}", g.pos);
        //println!("dir: {}", g.dir);
        fake_map[g.pos.x][g.pos.y] = '#';
        //pretty_print(&fake_map);

        let mut loop_detector = HashMap::new();
        //block = Block::Valid;
        direction = check_direction(g.dir);
        //println!("Guard: {}, {}, {direction} ", g.pos.x, g.pos.y);

        loop {
            match block {
                Block::Exit => {
                    //println!("Last pos: {guard_coord}");
                    //println!("Found exit");
                    break;
                }
                Block::Obstacle => {
                    direction = check_direction(direction);
                    move_guard(&mut guard_coord, &direction, &fake_map);
                }
                _ => {}
            }

            loop_detector
                .entry(guard_coord)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);

            let vals: Vec<usize> = loop_detector.values().cloned().collect();
            if vals.contains(&30) {
                count += 1;
                break;
            }

            block = move_guard(&mut guard_coord, &direction, &fake_map);
            //println!("Move: {},{},{}", guard_coord.x, guard_coord.y, direction);
            //thread::sleep(time::Duration::from_millis(200));
        }
        //panic!();
        println!("Iter: {i}");
    }

    print!("\nNum of steps: {}\n\n", count);
}

fn pretty_print(fake_map: &Vec<Vec<char>>) {
    for row in fake_map {
        let row: String = row.clone().iter().collect();
        println!("{row}");
    }
    println!();
}

fn move_guard(position: &mut Position, direction: &Direction, map: &Vec<Vec<char>>) -> Block {
    match direction {
        Direction::Up => {
            let block = check_block(*position, map, -1, 0);
            match block {
                Block::Valid => {
                    position.x -= 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Down => {
            let block = check_block(*position, map, 1, 0);
            match block {
                Block::Valid => {
                    position.x += 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Right => {
            let block = check_block(*position, map, 0, 1);
            match block {
                Block::Valid => {
                    position.y += 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
        Direction::Left => {
            let block = check_block(*position, map, 0, -1);
            match block {
                Block::Valid => {
                    position.y -= 1;
                    Block::Valid
                }
                Block::Obstacle => Block::Obstacle,
                _ => Block::Exit,
            }
        }
    }
}

fn check_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Down => Direction::Left,
        Direction::Right => Direction::Down,
        Direction::Left => Direction::Up,
    }
}

fn check_block(pos: Position, map: &Vec<Vec<char>>, xmove: i32, ymove: i32) -> Block {
    if xmove < 0 && pos.x == 0 {
        return Block::Exit;
    }

    if ymove < 0 && pos.y == 0 {
        return Block::Exit;
    }

    let next_x = (pos.x as i32 + xmove) as usize;
    if (pos.x as i32 + xmove) as usize == map.len() {
        return Block::Exit;
    }

    let next_y = (pos.y as i32 + ymove) as usize;
    if (pos.y as i32 + ymove) as usize == map[0].len() {
        return Block::Exit;
    }

    if map[next_x][next_y] == '#' {
        return Block::Obstacle;
    }

    return Block::Valid;
}
