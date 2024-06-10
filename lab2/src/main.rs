use std::fmt::Display;

mod chord_method;
mod golden_ratio_method;
mod halving_method;
mod newton_method;

const MAX_ITERATIONS: usize = 1000;

enum MethodError {
    Diverges,
}

struct Point {
    x: f64,
    y: f64,
}

struct Input {
    start: f64,
    end: f64,
    epsilon: f64,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: ({}, {})", self.x, self.y)
    }
}

impl Display for MethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodError::Diverges => write!(f, "Method diverges!"),
        }
    }
}

fn function(x: f64) -> f64 {
    x.powf(2_f64) + x + x.sin()
}

fn first_derevative(x: f64) -> f64 {
    2_f64 * x + 1_f64 + x.cos()
}

fn second_derevative(x: f64) -> f64 {
    2_f64 - x.sin()
}

fn main() {
    let input = Input {
        start: -1_f64,
        end: 0_f64,
        epsilon: 0.003_f64,
    };

    let result = input.chord_method();
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }
}
