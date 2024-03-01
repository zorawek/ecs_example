use rand::{random, Rng};

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Point {
    pub fn new() -> Self {
        Point {
            x: rand::thread_rng().gen_range(0..100),
            y: rand::thread_rng().gen_range(0..100),
        }
    }
}

impl Color {
    pub fn new() -> Self {
        Color {
            red: random(),
            green: random(),
            blue: random(),
        }
    }
}