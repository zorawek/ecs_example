#![allow(
    unused,
    dead_code
)]
use std::time::Instant;
use crate::extras::{Color, Point};

mod extras;

// OO
struct Ball {
    position: Point,
    color: Color,
    alt_color: Color
}

impl Ball {
    fn new() -> Self {
        Ball {
            position: Point::new(),
            color: Color::new(),
            alt_color: Color::new()
        }
    }
}
// DOD
struct Balls {
    positions: Vec<Point>,
    colors: Vec<Color>,
    alt_colors: Vec<Color>
}

impl Balls {
    fn new(limit: usize) -> Self {
        let mut positions = Vec::with_capacity(limit);
        let mut colors = Vec::with_capacity(limit);
        let mut alt_colors = Vec::with_capacity(limit);
        for _ in 0..limit {
            positions.push(Point::new());
            colors.push(Color::new());
            alt_colors.push(Color::new());
        }
        Balls {
            positions,
            colors,
            alt_colors
        }
    }
}

fn generate_oo_balls(limit: usize) -> Vec<Ball> {
    let mut balls = Vec::with_capacity(limit);
    for _ in 0..limit {
        balls.push(Ball::new());
    }
    return balls;
}

fn main() {
    println!("Starting ...");
    let limit = 5_000_000usize;
    let ticks = 1_000;

    // Generate OO balls
    let start = Instant::now();
    let mut balls = generate_oo_balls(limit);
    println!("Create OO balls: {:.2?}", start.elapsed());

    // Move OO balls
    let start = Instant::now();
    for _ in 0..ticks {
        for ball in balls.iter_mut() {
            ball.position.x += 2;
            ball.position.y += 3;
        }
    }
    println!("Move OO balls: {:.2?}", start.elapsed());

    // Test
    println!("Test: {}", balls.iter().map(|b| b.position.x + b.position.y).sum::<i32>());

    // Generate DOD balls
    let start = Instant::now();
    let mut balls = Balls::new(limit);
    println!("Create DOD balls: {:.2?}", start.elapsed());

    // Move DOD balls
    let start = Instant::now();
    for _ in 0..ticks {
        for p in balls.positions.iter_mut() {
            p.x += 2;
            p.y += 3;
        }
    }
    println!("Move DOD balls: {:.2?}", start.elapsed());

    // Test
    println!("Test: {}", balls.positions.iter().map(|p| p.x + p.y).sum::<i32>());
    println!("... and finished!");
}