extern crate piston_window;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
use std::time::{Duration, Instant};
use piston_window::*;
use piston_window::rectangle::margin;

const X_TILE_NUMBER: usize = 25;
const Y_TILE_NUMBER: usize = 25;

const TILE_SIZE: f32 = 10.0;
const TILE_MARGIN: f32 = 1.0;

const WINDOW_WIDTH: f32 = X_TILE_NUMBER as f32 * TILE_SIZE + TILE_MARGIN * X_TILE_NUMBER as f32;
const WINDOW_HEIGHT: f32 = Y_TILE_NUMBER as f32 * TILE_SIZE + TILE_MARGIN * Y_TILE_NUMBER as f32;


type Colour = [f32; 4];

const WHITE: Colour = [1.0, 1.0, 1.0, 1.0];
const BLACK: Colour = [0.0, 0.0, 0.0, 1.0];
const GREEN: Colour = [0.0, 0.5, 0.1, 1.0];
const GREEN_LIGHT: Colour = [0.5, 0.9, 0.5, 1.0];
const RED: Colour = [1.0, 0.0, 0.0, 1.0];

#[derive(Clone, PartialEq)]
enum TileType {
    EMPTY,
    WALL,
    CANDY,
    SNAKE_HEAD,
    SNAKE_BODY,
}

#[derive(Clone, PartialEq)]
enum SnakeDirection {
    NONE,
    UP,
    DOWN,
    RIGHT,
    LEFT,
}

#[derive(Clone)]
struct Tile {
    pub colour: Colour,
    pub tile_type: TileType,
}

impl Tile {
    pub fn set(&mut self, tile_type: TileType) {
        self.colour = Tile::get_colour_from_type(&tile_type);
        self.tile_type = tile_type;
    }

    fn get_colour_from_type(tile_type: &TileType) -> Colour {
        match tile_type {
            TileType::EMPTY => WHITE,
            TileType::WALL => BLACK,
            TileType::CANDY => RED,
            TileType::SNAKE_HEAD => GREEN,
            TileType::SNAKE_BODY => GREEN_LIGHT,
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            tile_type: TileType::EMPTY,
            colour: WHITE,
        }
    }
}

type Position = (usize, usize);

type Map = Vec<Vec<Tile>>;

fn build_map() -> Map {
    let map: Map = vec![vec![Tile::default(); Y_TILE_NUMBER as usize]; X_TILE_NUMBER as usize];

    set_wall(map)
}

fn set_wall(map: Map) -> Map {
    let mut cloned_map = map.clone();

    for i in 0..X_TILE_NUMBER {
        for j in 0..Y_TILE_NUMBER as usize {
            if i == 0 || i == X_TILE_NUMBER - 1 || j == 0 || j == Y_TILE_NUMBER - 1 {
                cloned_map[i][j].set(TileType::WALL);
            }
        }
    }
    cloned_map
}

fn set_candy(map: &mut Map) {
    let mut x = rand::random::<usize>() % X_TILE_NUMBER;
    let mut y = rand::random::<usize>() % Y_TILE_NUMBER;
    while map[x][y].tile_type != TileType::EMPTY {
        x = rand::random::<usize>() % X_TILE_NUMBER;
        y = rand::random::<usize>() % Y_TILE_NUMBER;
    }

    map[x][y].set(TileType::CANDY);
}

fn set_snake(map: &mut Map) -> Position {
    let x_snake = X_TILE_NUMBER / 2;
    let y_snake = Y_TILE_NUMBER / 2;
    map[x_snake][y_snake].set(TileType::SNAKE_HEAD);
    (x_snake, y_snake)
}

fn add_snake_part(map: &mut Map, snake_pos: Position, snake_parts: &mut Vec<Position>) {
    let new_part_pos;
    if snake_parts.is_empty() {
        new_part_pos = find_near_empty_tile(map, &snake_pos);
    } else {
        new_part_pos = find_near_empty_tile(map, snake_parts.last().unwrap())
    }
    map[new_part_pos.0][new_part_pos.1].set(TileType::SNAKE_BODY);
    snake_parts.push(new_part_pos);
}

fn find_near_empty_tile(map: &Map, target: &Position) -> Position {
    return if map[target.0][target.1 - 1].tile_type == TileType::EMPTY {
        (target.0, target.1 - 1)
    } else if map[target.0 + 1][target.1].tile_type == TileType::EMPTY {
        (target.0 + 1, target.1)
    } else if map[target.0 - 1][target.1].tile_type == TileType::EMPTY {
        (target.0, target.1)
    } else {
        (target.0, target.1 + 1)
    };
}

fn main() {
    println!("[INFO] Creating a widow of {}x{}px", WINDOW_WIDTH, WINDOW_HEIGHT);

    let mut window: PistonWindow =
        WindowSettings::new("Rustian Snake", [WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64])
            .exit_on_esc(true).build().unwrap();

    let mut map = build_map();
    let mut score = 0;

    let mut snake_pos = set_snake(&mut map);
    let mut snake_parts: Vec<Position> = Vec::new();
    set_candy(&mut map);

    let mut snake_direction = SnakeDirection::NONE;
    let mut last_process = Instant::now();

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            snake_direction = match key {
                Key::Z => SnakeDirection::UP,
                Key::Up => SnakeDirection::UP,
                Key::S => SnakeDirection::DOWN,
                Key::Down => SnakeDirection::DOWN,
                Key::Q => SnakeDirection::LEFT,
                Key::Left => SnakeDirection::LEFT,
                Key::D => SnakeDirection::RIGHT,
                Key::Right => SnakeDirection::RIGHT,
                _ => snake_direction
            };
        };

        if last_process.elapsed().as_millis() >= 500 {
            // Move Snake
            let next_pos: Position = match snake_direction {
                SnakeDirection::NONE => (snake_pos.0, snake_pos.1),
                SnakeDirection::UP => (snake_pos.0, snake_pos.1 - 1 as usize),
                SnakeDirection::DOWN => (snake_pos.0, snake_pos.1 + 1 as usize),
                SnakeDirection::RIGHT => (snake_pos.0 + 1 as usize, snake_pos.1),
                SnakeDirection::LEFT => (snake_pos.0 - 1 as usize, snake_pos.1),
            };

            let next_tile = map[next_pos.0][next_pos.1].clone();

            if next_tile.tile_type == TileType::WALL || next_tile.tile_type == TileType::SNAKE_BODY {
                println!("Game over. Score: {}", score);
                return;
            }

            if next_tile.tile_type == TileType::CANDY {
                score += 1;
                map[next_pos.0][next_pos.1].set(TileType::EMPTY);
                set_candy(&mut map);
                println!("Score: {}", score);

                add_snake_part(&mut map, snake_pos, &mut snake_parts);
            }

            map[snake_pos.0][snake_pos.1].set(TileType::EMPTY);
            map[next_pos.0][next_pos.1].set(TileType::SNAKE_HEAD);

            let mut previous_pos = snake_pos;

            for i in 0..snake_parts.len() {
                map[previous_pos.0][previous_pos.1].set(TileType::SNAKE_BODY);
                map[ snake_parts[i].0][ snake_parts[i].1].set(TileType::EMPTY);

                let tmp = previous_pos;
                previous_pos =  snake_parts[i];
                snake_parts[i] = tmp;
            }

            snake_pos = next_pos;

            last_process = Instant::now();
        }

        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0; 4], graphics);
            for (x, column) in map.iter().enumerate() {
                for (y, tile) in column.iter().enumerate() {
                    let x_rect = x as f64 * TILE_SIZE as f64 + x as f64 * TILE_MARGIN as f64 + TILE_MARGIN as f64;
                    let y_rect = y as f64 * TILE_SIZE as f64 + y as f64 * TILE_MARGIN as f64 + TILE_MARGIN as f64;
                    rectangle(tile.colour,
                              [x_rect, y_rect, TILE_SIZE as f64, TILE_SIZE as f64],
                              context.transform,
                              graphics);
                }
            }
        });
    }
}