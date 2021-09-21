use ::rand::Rng;
use macroquad::{color::Color, prelude::*};

#[derive(PartialEq, Copy, Clone, std::fmt::Debug)]
enum Direction {
    North,
    East,
    West,
    South,
}

#[derive(Copy, Clone)]
struct Square {
    x: f32,
    y: f32,
    size: f32,
    color: Color,
    direction: Direction,
    speed: f32,
}

impl Square {
    fn new(x: f32, y: f32, size: f32, color: Color, direction: Direction, speed: f32) -> Self {
        Self {
            x,
            y,
            size,
            color,
            direction,
            speed,
        }
    }

    fn render(&mut self) {
        match self.direction {
            Direction::South => self.y += self.speed,
            Direction::North => self.y -= self.speed,
            Direction::East => self.x += self.speed,
            Direction::West => self.x -= self.speed,
        }
        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
    }

    fn distance_from(&self, other: Self) -> f32 {
        (((other.x - self.x) * (other.x - self.x)) + ((other.y - self.y) * (other.y - self.y)))
            .sqrt() as f32
    }
}

const FOOD_SIZE: f32 = 25.0;
const PLAYER_SIZE: f32 = 30.0;

#[macroquad::main("AI Box by Vahin Sharma")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    let mut food = Square::new(
        rng.gen::<f32>() * screen_width(),
        rng.gen::<f32>() * screen_height(),
        FOOD_SIZE,
        RED,
        Direction::North,
        0.,
    );
    let mut player = Square::new(
        rng.gen::<f32>() * screen_width(),
        rng.gen::<f32>() * screen_height(),
        PLAYER_SIZE,
        BLUE,
        Direction::East,
        1.,
    );

    let mut next_possible_directions: Vec<Direction> = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    let mut previous_player_food_distance = player.distance_from(food);

    loop {
        clear_background(Color::from_rgba(224, 255, 209, 1));

        food.render();

        // If next_possible_directions is empty, then relocate the food
        if next_possible_directions.is_empty() {
            food.x = rng.gen::<f32>() * screen_width();
            food.y = rng.gen::<f32>() * screen_height();
            previous_player_food_distance = player.distance_from(food);
            next_possible_directions = vec![
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
            ];
        }

        // Getting a random item from next_possible_directions and then setting it as player's direction
        player.direction =
            next_possible_directions[rng.gen_range(0..next_possible_directions.len())];

        player.render();

        let player_food_distance = player.distance_from(food);
        // If the player moves away from the food then remove that direction from next_possible_directions
        if player_food_distance > previous_player_food_distance {
            next_possible_directions.retain(|&x| x != player.direction);
        }

        previous_player_food_distance = player_food_distance;

        println!(
            "{} {:?}",
            previous_player_food_distance, next_possible_directions
        );

        next_frame().await
    }
}
