extern crate termion;

use rand::Rng;
use std::fs;
use std::io;
use std::io::{stdin, stdout, Read, Write};
use termion::raw::IntoRawMode;
use termion::{async_stdin, clear, color, cursor, style};
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
enum Action {
Drive(u8),
Put
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy)]
enum Terrain {
    Slope(Direction),
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
                    ">" => Terrain::Slope(Direction::East),
                    "<" => Terrain::Slope(Direction::West),
                    "^" => Terrain::Slope(Direction::North),
                    "v" => Terrain::Slope(Direction::South),
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
        Terrain::Slope(direction) => {
            let new_coord = match direction {
                Direction::East => Point {
                    x: coord.x,
                    y: coord.y + 1,
                },
                Direction::West => Point {
                    x: coord.x,
                    y: coord.y.saturating_sub(1),
                },
                Direction::North => Point {
                    x: coord.x.saturating_sub(1),
                    y: coord.y,
                },
                Direction::South => Point {
                    x: coord.x + 1,
                    y: coord.y,
                },
            };
            land(&new_coord, world) 
        }
        _ => Point {
            x: coord.x,
            y: coord.y,
        },
    }
}

fn shot(from: &Point, dir: &Point, str: i32) -> Point {
    Point {
        x: from.x + str * dir.x,
        y: from.y + str * dir.y,
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
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "O").unwrap();
            }
            Terrain::Sand => {
                write!(stdout, "{}", color::Fg(color::LightYellow));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "@").unwrap();
            }
            Terrain::Water => {
                write!(stdout, "{}", color::Fg(color::LightBlue));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "w").unwrap();
            }
            Terrain::Slope(Direction::East) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), ">").unwrap();
            }
            Terrain::Slope(Direction::West) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "<").unwrap();
            }
            Terrain::Slope(Direction::North) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "^").unwrap();
            }
            Terrain::Slope(Direction::South) => {
                write!(stdout, "{}", color::Fg(color::LightGreen));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "v").unwrap();
            }
            _ => {
                write!(stdout, "{}", color::Fg(color::Green));
                write!(stdout, "{}{}", cursor::Goto(x + 1, y + 1), "@").unwrap();
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

        println!("Strength: {}", strength);
        println!("Please input a direction:");
        let mut direction = String::new();

        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");

        let dir: Point = match direction.trim() {
            // Trim whitespace
            "N" => Point { x: -1, y: 0 },
            "S" => Point { x: 1, y: 0 },
            "E" => Point { x: 0, y: 1 },
            "W" => Point { x: 0, y: -1 },
            "NW" => Point { x: -1, y: -1 },
            "NE" => Point { x: -1, y: 1 },
            "SE" => Point { x: 1, y: 1 },
            "SW" => Point { x: 1, y: -1 },
            _ => {
                Point { x: 0, y: 0 };
                break;
            }
        };
        let fall: Point = shot(&ball, &dir, strength);
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
