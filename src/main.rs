extern crate termion;

use rand::Rng;
use std::fs;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color, cursor, style};
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
enum Action {
    Drive(Point, i32),
    Put(Point),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Direction;
impl Direction {
    pub const N: Point = Point { x: -1, y: 0 };
    pub const S: Point = Point { x: 1, y: 0 };
    pub const W: Point = Point { x: 0, y: -1 };
    pub const E: Point = Point { x: 0, y: 1 };
    pub const NW: Point = Point { x: -1, y: -1 };
    pub const NE: Point = Point { x: -1, y: 1 };
    pub const SW: Point = Point { x: 1, y: -1 };
    pub const SE: Point = Point { x: 1, y: 1 };
}

#[derive(Debug, Clone, Copy)]
enum Terrain {
    Slope(Point),
    Tree,
    Sand,
    Hole,
    Fairway,
    Water,
    Grass,
    Start(u8),
}
#[derive(Debug, Clone)]
struct World {
    width: i32,
    height: i32,
    start: Point,
    terrain_map: Vec<Terrain>,
}
impl World {
    fn get_terrain(&self, coord: &Point) -> &Terrain {
        let idx: usize = coord.x as usize * self.width as usize + coord.y as usize;
        &self.terrain_map[idx]
    }

    fn from_file(file_path: String) -> World {
        let mut t: Vec<Terrain> = vec![];
        let mut row_counter: i32 = 0;
        let mut start: Point = Point { x: 0, y: 0 };
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let lines = contents.lines();
        for line in lines {
            row_counter += 1;
            let mut col_counter: i32 = 0;
            for c in line.trim().split(" ") {
                col_counter += 1;
                let terrain = match c {
                    ">" => Terrain::Slope(Direction::E),
                    "<" => Terrain::Slope(Direction::W),
                    "^" => Terrain::Slope(Direction::N),
                    "v" => Terrain::Slope(Direction::S),
                    "o" => Terrain::Hole,
                    "t" => Terrain::Tree,
                    "s" => Terrain::Sand,
                    "x" => {
                        start.x = row_counter;
                        start.y = col_counter;
                        Terrain::Start(0)
                    }
                    "w" => Terrain::Water,
                    "f" => Terrain::Fairway,
                    _ => Terrain::Grass,
                };
                t.push(terrain)
            }
        }
        return World {
            width: t.len() as i32 / row_counter,
            height: row_counter,
            start,
            terrain_map: t,
        };
    }
}

fn land(coord: &Point, world: &World) -> Point {
    let terrain = world.get_terrain(&coord);
    match terrain {
        Terrain::Slope(direction) => Point {
            x: coord.x + direction.x,
            y: coord.y + direction.y,
        },
        _ => Point {
            x: coord.x,
            y: coord.y,
        },
    }
}

fn shot(from: &Point, action: Action) -> Point {
    let (dir, strength) = match action {
        Action::Put(d) => (d, 1),
        Action::Drive(d, strength) => (d, strength),
    };

    Point {
        x: from.x + (strength as i32) * dir.x,
        y: from.y + (strength as i32) * dir.y,
    }
}

fn draw_map(world: &World) {
    let width: u16 = world.width as u16;
    let height: u16 = world.height as u16;

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    let mut i: u16 = 0;
    for terrain in &world.terrain_map {
        let x = i % width;
        let y = i / width;
        i += 1;
        match terrain {
            Terrain::Fairway => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "@").unwrap();
            }
            Terrain::Start(n) => {
                write!(stdout, "{}", color::Fg(color::Rgb(255, 255, 255)));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), n).unwrap();
            }
            Terrain::Tree => {
                write!(stdout, "{}", color::Fg(color::Rgb(0, 255, 0)));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "Y").unwrap();
            }
            Terrain::Hole => {
                write!(stdout, "{}", color::Fg(color::Red));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "F").unwrap();
            }
            Terrain::Sand => {
                write!(stdout, "{}", color::Fg(color::LightYellow));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "*").unwrap();
            }
            Terrain::Water => {
                write!(stdout, "{}", color::Fg(color::LightBlue));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "w").unwrap();
            }
            Terrain::Slope(Direction::E) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), ">").unwrap();
            }
            Terrain::Slope(Direction::W) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "<").unwrap();
            }
            Terrain::Slope(Direction::N) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "^").unwrap();
            }
            Terrain::Slope(Direction::S) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "v").unwrap();
            }
            _ => {
                write!(stdout, "{}", color::Fg(color::Green));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "#").unwrap();
            }
        }
    }
}

fn main() {
    let mut world = World::from_file("map.txt".to_string());
    let mut ball = world.start.clone();
    let mut plays = 1;
    loop {
        draw_map(&world);
        let strength = rand::thread_rng().gen_range(1..=6)
            + match (world.get_terrain(&ball)) {
                Terrain::Sand => -1,
                Terrain::Fairway => 1,
                _ => 0,
            };

        println!("\n Strength: {} (1 if put)", strength);
        println!("Please input a direction:");
        let mut direction = String::new();
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");

        let dir: Point = match direction.trim() {
            // Trim whitespace
            "N" => Direction::N,
            "S" => Direction::S,
            "E" => Direction::E,
            "W" => Direction::W,
            "NW" => Direction::NW,
            "NE" => Direction::NE,
            "SE" => Direction::SE,
            "SW" => Direction::SW,
            _ => {
                Point { x: 0, y: 0 };
                continue;
            }
        };
        println!("(D)rive or (P)ut?");
        let mut club = String::new();
        io::stdin()
            .read_line(&mut club)
            .expect("Failed to read line");
        let action: Action = match club.trim() {
            "P" => Action::Put(dir),
            "D" => Action::Drive(dir, strength),
            _ => continue,
        };

        Action::Drive(dir, strength);
        let fall: Point = shot(&ball, action);
        let land_spot = land(&fall, &world);
        let land_terrain = world.get_terrain(&land_spot);
        match land_terrain {
            Terrain::Hole => {
                println!("Hit hole in {} shots", plays);
                break;
            }
            Terrain::Water => {
                println!("Redo: Water");
            }
            Terrain::Tree => {
                println!("Redo: Tree");
            }
            _ => {
                let idx: usize = land_spot.x as usize * world.width as usize + land_spot.y as usize;
                world.terrain_map[idx] = Terrain::Start(plays);
                ball = land_spot;
            }
        }

        plays += 1;
    }
}
