use std::fmt::Display;

use gradient_method::gradient_method;

use crate::fast_descend_method::fast_descend_method;

mod fast_descend_method;
mod gradient_method;

#[derive(Debug)]
enum MethodError {
    Diverges,
}

struct Input {
    epsilon: f64,
    first_approximation: Point2D,
    iteration_step: f64,
}

const MAX_ITERATIONS: usize = 1000;

#[derive(Clone, Copy)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point({}, {})", self.x, self.y)
    }
}

impl Point2D {
    pub fn zero() -> Self {
        Point2D { x: 0., y: 0. }
    }

    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    fn compute(x: f64, y: f64, calc: impl FnOnce(f64, f64) -> f64) -> Self {
        Point {
            x,
            y,
            z: calc(x, y),
        }
    }
}

impl From<Point> for Point2D {
    fn from(value: Point) -> Self {
        Point2D {
            x: value.x,
            y: value.y,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: ({:.5}, {:.5}, {:.5})", self.x, self.y, self.z)
    }
}

fn function(x: f64, y: f64) -> f64 {
    f64::exp(-2f64 * x.powi(2) - 5f64 * y.powi(2) + x * y)
}

fn first_derivative_x(x: f64, y: f64) -> f64 {
    -4f64 * function(x, y) * x - function(x, y) * y
}

fn first_derivative_y(x: f64, y: f64) -> f64 {
    -10f64 * function(x, y) * y - function(x, y) * x
}

fn gradient(Point2D { x, y }: Point2D) -> Point2D {
    Point2D::new(first_derivative_x(x, y), first_derivative_y(x, y))
}

fn main() {
    let input = Input {
        epsilon: 0.0000001,
        first_approximation: Point2D::new(-1., -1.),
        iteration_step: 0.1,
    };

    let fast_descend_input = Input {
        epsilon: 0.00000001,
        first_approximation: Point2D::new(-0.1, -0.1),
        iteration_step: 0.05,
    };
    let max_point = gradient_method(input);
    // let max_point = fast_descend_method(fast_descend_input);
    println!("{max_point}");
}
