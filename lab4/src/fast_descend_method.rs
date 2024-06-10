use core::panic;
use std::ops::{Add, Mul, Neg, Sub};

use crate::{function, gradient, Input, MethodError, Point, Point2D, MAX_ITERATIONS};

impl Point2D {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2))
    }

    pub fn unit_vector(&self) -> Point2D {
        Point2D {
            x: self.x / self.length(),
            y: self.y / self.length(),
        }
    }
}

impl Neg for Point2D {
    type Output = Point2D;

    fn neg(self) -> Self::Output {
        Point2D {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<f64> for Point2D {
    type Output = Point2D;

    fn mul(self, rhs: f64) -> Self::Output {
        Point2D {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Point2D> for f64 {
    type Output = Point2D;

    fn mul(self, rhs: Point2D) -> Self::Output {
        Point2D {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

pub fn fast_descend_method(input: Input) -> Point {
    let mut point = input.first_approximation;
    for step in 0..MAX_ITERATIONS {
        println!("{:-^80}", step + 1);
        println!("point: {point}");
        let gradient_vector = gradient(point);
        println!("gradient: {gradient_vector}");
        println!("gradient vector length: {}", gradient_vector.length());
        if gradient_vector.length() < input.epsilon {
            return Point::compute(point.x, point.y, function);
        }
        println!("gradient length < epsilon: {}", input.epsilon);

        let gradient_unit_vector = gradient_vector.unit_vector();
        println!("gradient unit vector {gradient_unit_vector}");
        let h = halving_method(point, gradient_unit_vector, -10., 10., input.epsilon)
            .expect("method diverges");
        println!("h {h}");

        point = point + h * gradient_unit_vector;
    }

    panic!("diverge");
}

fn halving_method(
    start_point: Point2D,
    gradient: Point2D,
    mut start: f64,
    mut end: f64,
    epsilon: f64,
) -> Result<f64, MethodError> {
    let mut count: usize = 0;
    while (end - start) > (2_f64 * epsilon) {
        if count >= MAX_ITERATIONS {
            return Err(MethodError::Diverges);
        }

        let h1: f64 = (start + end - epsilon) / 2_f64;
        let h2: f64 = (start + end + epsilon) / 2_f64;
        let z1 = function(
            start_point.x + h1 * gradient.x,
            start_point.y + h1 * gradient.y,
        );
        let z2 = function(
            start_point.x + h2 * gradient.x,
            start_point.y + h2 * gradient.y,
        );

        if z1 < z2 {
            start = h1;
        } else {
            end = h2;
        }

        count += 1;
    }

    let hm = (start + end) / 2_f64;

    Ok(hm)
}
