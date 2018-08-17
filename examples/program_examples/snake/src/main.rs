extern crate crossterm;
extern crate rand;

mod map;
mod snake;
mod variables;
mod messages;

use self::crossterm::input::input;
use self::crossterm::terminal::{terminal, ClearType};
use self::crossterm::style::Color;

use self::crossterm::{Screen, Crossterm};

use map::Map;
use variables::{Size, Direction, Position};
use snake::Snake;

use std::collections::HashMap;
use std::{thread, time};
use std::iter::Iterator;
use std::io::Read;

fn main() {
    let map_size = title_screen();

    {
        let mut screen = Screen::new(true);
        let crossterm = Crossterm::new(&screen);

        let cursor = crossterm.cursor();
        let mut input = crossterm.input();

        cursor.hide();
        let mut stdin = input.read_async().bytes();

        let mut free_positions: HashMap<String, Position> = HashMap::with_capacity((map_size.width * map_size.height) as usize);

        let mut map = Map::new(map_size.clone());
        map.render_map(&screen, &mut free_positions);

        let mut direction = Direction::Right;

        let mut snake = Snake::new(map_size.clone());

        for part in snake.get_parts().iter() {
            free_positions.remove_entry(format!("{},{}", part.position.x, part.position.y).as_str());
        }

        map.spawn_food(&free_positions, &screen);

        loop {
            thread::sleep(time::Duration::from_millis(500));
            let pressed_key = stdin.next();

            if let Some(Ok(key)) = pressed_key {
                match key as char {
                    'w' => direction = Direction::Up,
                    'a' => direction = Direction::Left,
                    's' => direction = Direction::Down,
                    'd' => direction = Direction::Right,
                    _ => {}
                }
            }

            snake.move_snake(&direction, &screen, &mut free_positions);

            if map.is_out_of_bounds(snake.snake_parts[0].position)
            {
                break;
            }

            snake.draw_snake(&screen);



            if snake.has_eaten_food(map.foot_pos)
            {
                map.spawn_food(&free_positions, &screen);
            }


        }
    }
    game_over_screen();
}

fn title_screen() -> Size
{
    let crossterm = Crossterm::new(&Screen::default());

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal().clear(ClearType::All);

    println!("{}",messages::SNAKERS.join("\n\r"));
    cursor.goto(0, 15);
    println!("Enter map width:");
    cursor.goto(17, 15);
    let width = crossterm.input().read_line().unwrap();
    println!("\n\rEnter map height:");
    cursor.goto(17, 16);
    let height = crossterm.input().read_line().unwrap();

    let parsed_width = width.parse::<usize>().unwrap();
    let parsed_height = height.parse::<usize>().unwrap();

    let terminal = crossterm.terminal().clear(ClearType::All);

    return Size::new(parsed_width, parsed_height);
}

fn print_game_stats(map_size: Size, snake_lenght: usize, food_aten: usize, screen: &mut Screen)
{
    let crossterm = Crossterm::new(&Screen::default());

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal().clear(ClearType::All);

    screen.write(format!("Snake Lenght: {}\n\r", snake_lenght).as_ref());
    screen.write(format!("Food aten: {}\n\r", snake_lenght).as_ref());

    cursor.goto(0,map_size.height as u16);
    cursor.goto(0,map_size.height as u16);
}

fn game_over_screen()
{
    let crossterm = Crossterm::new(&Screen::default());

    let cursor = crossterm.cursor();
    let terminal = crossterm.terminal();

    terminal.clear(ClearType::All);

    println!("{}",messages::END_MESSAGE.join("\n\r"));
//    cursor.goto()
    cursor.show();
}


